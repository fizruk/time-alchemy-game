use geng::Sound;

use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        for enemy in &mut self.level_map.enemies {
            enemy.animation_clock += delta_time;
            if let EnemyState::Action(Cooldown {
                action, leftover, ..
            }) = &mut enemy.state
            {
                *leftover -= delta_time;
                if *leftover <= r32(0.0) {
                    enemy.state = EnemyState::Idle;
                    enemy.animation_clock = r32(0.0);
                }
            }
        }

        for enemy in &mut self.level_map.dead_enemies {
            enemy.animation_clock += delta_time;
            if let EnemyState::Action(Cooldown {
                action, leftover, ..
            }) = &mut enemy.state
            {
                *leftover -= delta_time;
            }
        }
        self.level_map
            .dead_enemies
            .retain(|enemy| match enemy.state {
                EnemyState::Idle => false,
                EnemyState::Action(Cooldown { leftover, .. }) => leftover > r32(0.0),
            })
    }

    pub fn player_input(&mut self, action: Action) {
        match self.state {
            State::Day => self.player_input_day(action),
            State::ExpandMap => self.player_input_expand_map(action),
            State::Shop => todo!(),
            State::Night => self.player_input_night(action),
        }
    }

    pub fn player_input_expand_map(&mut self, action: Action) {
        match action {
            Action::MoveTo(pos) => {
                if !self.level_map.adjacent(pos) {
                    return;
                }
                self.level_map.expansion_cells.push(pos);
                self.phase_day();

                if let Some(cell) = self
                    .level_map
                    .cells_iter()
                    .filter(|cell| {
                        !(*cell == self.player.pos
                            || self.level_map.items.iter().any(|item| *cell == item.pos))
                    })
                    .choose(&mut thread_rng())
                {
                    self.level_map.enemies.push(Enemy::new(cell));
                    self.effects.push(Effect::PlaySound(SoundKind::RobotMove));
                }
            }
            Action::MoveUp => {}
            Action::MoveDown => {}
            Action::MoveLeft => {}
            Action::MoveRight => {}
        }
    }

    pub fn player_input_day(&mut self, action: Action) {
        let delta = match action {
            Action::MoveDown => vec2(0, -1),
            Action::MoveUp => vec2(0, 1),
            Action::MoveLeft => vec2(-1, 0),
            Action::MoveRight => vec2(1, 0),
            Action::MoveTo(pos) => pos - self.player.pos,
        };
        let target_pos = self.player.pos + delta;

        if delta.x.abs() + delta.y.abs() != 1 {
            return;
        }

        if !self.level_map.inside(target_pos) {
            return;
        }

        let target_on_enemy = self
            .level_map
            .enemies
            .iter()
            .any(|enemy| enemy.pos == target_pos);
        if target_on_enemy {
            return;
        }

        let (target_items, other_items) = self
            .level_map
            .items
            .drain(..)
            .partition(|item| item.pos == target_pos);
        self.level_map.items = other_items;
        let mut did_hit_enemy = false;
        for mut item in target_items {
            match item.kind {
                ItemKind::Sword { damage } => {
                    for enemy in &mut self.level_map.enemies {
                        enemy.take_damage(damage);
                        did_hit_enemy = true;
                    }
                    let (live, dead) = self
                        .level_map
                        .enemies
                        .drain(..)
                        .partition(|enemy| enemy.health > 0);
                    self.level_map.enemies = live;
                    self.level_map.dead_enemies = dead;

                    item.pos = self.player.pos;
                    self.level_map.items.push(item);
                }
                ItemKind::Ingredient(ingredient) => {
                    self.player.backpack.ingredients.push(ingredient)
                }
            }
        }

        self.player.pos = target_pos;
        self.camera.center = self.player.pos.map(|x| x as f32);

        self.effects.push(Effect::PlaySound(SoundKind::TwoSteps));
        if did_hit_enemy {
            self.effects.push(Effect::PlaySound(SoundKind::MetalHit));
        }

        if self.level_map.enemies.is_empty() {
            self.state = State::ExpandMap;
        }

        self.player.turns_remaining = self.player.turns_remaining.saturating_sub(1);
        if self.player.turns_remaining == 0 {
            self.phase_night();
        }
    }

    fn player_input_night(&mut self, _action: Action) {}

    fn phase_day(&mut self) {
        self.state = State::Day;
        self.player.turns_remaining = 3;
    }

    fn phase_night(&mut self) {
        let mut rng = thread_rng();
        self.state = State::Night;

        let ids: Vec<_> = (0..self.level_map.enemies.len()).collect();
        for id in ids {
            let Some(enemy) = self.level_map.enemies.get(id) else {
                continue;
            };

            let Some(target_position) = [vec2(-1, 0), vec2(0, -1), vec2(1, 0), vec2(0, 1)]
                .into_iter()
                .map(|delta| enemy.pos + delta)
                .filter(|&pos| {
                    self.level_map.inside(pos)
                        && self.player.pos != pos
                        && !self.level_map.enemies.iter().any(|other| other.pos == pos)
                        && !self.level_map.items.iter().any(|item| item.pos == pos)
                })
                .choose(&mut rng)
            else {
                continue;
            };

            let enemy = self
                .level_map
                .enemies
                .get_mut(id)
                .expect("enemy was confirmed to exist at the start of the loop");
            enemy.pos = target_position;
        }

        self.phase_day();
    }
}
