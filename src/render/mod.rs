use crate::{assets::Assets, model::*};

use geng::{prelude::*, TextAlign};

#[allow(dead_code)]
pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        for pos in model.level_map.cells_iter() {
            let x = pos.x as f32;
            let y = pos.y as f32;
            self.geng.draw2d().quad(
                framebuffer,
                &model.camera,
                Aabb2::point(vec2(x, y)).extend_symmetric(vec2(0.45, 0.45)),
                Rgba {
                    r: 0.2,
                    g: 0.2,
                    b: 0.2,
                    a: 1.0,
                },
            );
        }

        if let State::ExpandMap = model.state {
            for pos in model.level_map.adjacent_cells_iter() {
                let x = pos.x as f32;
                let y = pos.y as f32;
                self.geng.draw2d().quad(
                    framebuffer,
                    &model.camera,
                    Aabb2::point(vec2(x, y)).extend_symmetric(vec2(0.45, 0.45)),
                    Rgba {
                        r: 0.3,
                        g: 0.3,
                        b: 0.1,
                        a: 0.5,
                    },
                );
            }
        }

        let player_pos = model.player.pos.map(|x| x as f32);
        self.geng.draw2d().textured_quad(
            framebuffer,
            &model.camera,
            Aabb2::point(player_pos).extend_symmetric(vec2(0.4, 0.4)),
            &self.assets.sprites.player,
            Rgba::WHITE,
        );

        for item in &model.level_map.items {
            let item_pos = item.pos.map(|x| x as f32);
            match &item.kind {
                ItemKind::Ingredient(ingredient) => match ingredient {
                    Ingredient::Leaf => self.geng.draw2d().textured_quad(
                        framebuffer,
                        &model.camera,
                        Aabb2::point(item_pos).extend_symmetric(vec2(0.3, 0.3)),
                        &self.assets.sprites.leaf,
                        Rgba::WHITE,
                    ),
                    _ => {}
                },
                ItemKind::Sword { damage } => {
                    self.geng.draw2d().textured_quad(
                        framebuffer,
                        &model.camera,
                        Aabb2::point(item_pos).extend_symmetric(vec2(0.3, 0.3)),
                        &self.assets.sprites.sword,
                        Rgba::WHITE,
                    );
                    self.geng.default_font().draw(
                        framebuffer,
                        &model.camera,
                        &format!("{}", damage),
                        vec2(TextAlign::RIGHT, TextAlign::BOTTOM),
                        mat3::translate(item_pos + vec2(0.4, -0.4)) * mat3::scale_uniform(0.4),
                        Rgba::BLACK,
                    )
                }
            }
        }

        for enemy in model
            .level_map
            .enemies
            .iter()
            .chain(model.level_map.dead_enemies.iter())
        {
            let enemy_pos = enemy.pos.map(|x| x as f32);

            let animation = match &enemy.state {
                EnemyState::Idle => match enemy.mode {
                    EnemyMode::Normal => &self.assets.sprites.level_1_bot_idle_normal,
                    EnemyMode::Damaged => &self.assets.sprites.level_1_bot_idle_damaged,
                },
                EnemyState::Action(Cooldown { action, .. }) => match action {
                    EnemyAction::TakeDamage => &self.assets.sprites.level_1_bot_idle_damaged,
                    EnemyAction::Attack => &self.assets.sprites.level_1_bot_idle_normal,
                    EnemyAction::Die => &self.assets.sprites.level_1_bot_die,
                    EnemyAction::Spawn => &self.assets.sprites.level_1_bot_idle_normal,
                },
            };

            let total_duration = animation.iter().map(|frame| frame.duration).sum();
            let clock = match enemy.state {
                EnemyState::Action(Cooldown {
                    leftover, total, ..
                }) => r32(total_duration) * (r32(1.0) - leftover / total),
                EnemyState::Idle => enemy.animation_clock,
            };

            let animation_clock_rem =
                clock - (clock / r32(total_duration)).floor() * r32(total_duration);

            let mut t = r32(0.0);
            let mut target_frame = &animation[0];
            for frame in animation {
                if animation_clock_rem >= t && animation_clock_rem < t + r32(frame.duration) {
                    target_frame = frame;
                    break;
                } else {
                    t += r32(frame.duration);
                }
            }
            self.geng.draw2d().textured_quad(
                framebuffer,
                &model.camera,
                Aabb2::point(enemy_pos).extend_symmetric(vec2(0.45, 0.45)),
                &target_frame.texture,
                Rgba::WHITE,
            );
            if enemy.health > 0 {
                self.geng.default_font().draw(
                    framebuffer,
                    &model.camera,
                    &format!("{}", enemy.health),
                    vec2(TextAlign::RIGHT, TextAlign::BOTTOM),
                    mat3::translate(enemy_pos + vec2(0.4, -0.4)) * mat3::scale_uniform(0.4),
                    Rgba::RED,
                )
            }
        }
    }
}
