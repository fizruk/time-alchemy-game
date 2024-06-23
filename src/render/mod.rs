use crate::{assets::Assets, model::*};

use geng::prelude::*;

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
        self.geng.draw2d().quad(
            framebuffer,
            &model.camera,
            Aabb2::point(vec2(-0.5, -0.5)).extend_positive(size2),
            Rgba::BLUE,
        );
        for i in 0..model.level_map.size.x {
            for j in 0..model.level_map.size.y {
                let x = i as f32;
                let y = j as f32;
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
                ItemKind::Sword { damage, cost } => self.geng.draw2d().textured_quad(
                    framebuffer,
                    &model.camera,
                    Aabb2::point(item_pos).extend_symmetric(vec2(0.3, 0.3)),
                    &self.assets.sprites.sword,
                    Rgba::WHITE,
                ),
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
        }
    }
}
