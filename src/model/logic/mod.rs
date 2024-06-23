use geng::Sound;

use super::*;

impl Model {
    pub fn update(&mut self, _delta_time: Time) {}

    pub fn player_input(&mut self, action: Action) {
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

        let target_on_map = target_pos.x >= 0
            && target_pos.x < self.level_map.size.x
            && target_pos.y >= 0
            && target_pos.y < self.level_map.size.y;
        if !target_on_map {
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
        for mut item in target_items {
            match item.kind {
                ItemKind::Sword { damage } => {
                    for enemy in &mut self.level_map.enemies {
                        enemy.health -= damage;
                    }
                    self.level_map.enemies.retain(|enemy| enemy.health > 0);
                    item.pos = self.player.pos;
                    self.level_map.items.push(item);
                }
                ItemKind::Ingredient(ingredient) => {
                    self.player.backpack.ingredients.push(ingredient)
                }
            }
        }

        self.player.pos = target_pos;
        self.effects.push(Effect::PlaySound(SoundKind::Steps))
    }
}
