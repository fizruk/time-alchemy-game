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
        let size2 = model.level_map.size.map(|x| x as f32);
        for pos in model.level_map.cells_iter() {
            let x = pos.x as f32;
            let y = pos.y as f32;
            self.geng.draw2d().quad(
                framebuffer,
                &model.camera,
                Aabb2::point(vec2(x, y)).extend_symmetric(vec2(0.45, 0.45)),
                Rgba {
                    r: 0.9,
                    g: 0.9,
                    b: 0.9,
                    a: 1.0,
                },
            );
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

        for enemy in &model.level_map.enemies {
            let enemy_pos = enemy.pos.map(|x| x as f32);
            self.geng.draw2d().textured_quad(
                framebuffer,
                &model.camera,
                Aabb2::point(enemy_pos).extend_symmetric(vec2(0.3, 0.3)),
                &self.assets.sprites.enemy,
                Rgba::WHITE,
            );
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
