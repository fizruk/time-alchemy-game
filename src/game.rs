use crate::{assets::Assets, model::*, render::GameRender};

use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    model: Model,
    framebuffer_size: vec2<usize>,
    cursor_pos: vec2<f64>,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            model: Model::new(),
            framebuffer_size: vec2(1, 1), // dummy
            cursor_pos: vec2(0.0, 0.0),   // dummy
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
        self.framebuffer_size = framebuffer.size();
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyPress { key } => match key {
                geng::Key::ArrowDown => self.model.player_input(Action::MoveDown),
                geng::Key::ArrowUp => self.model.player_input(Action::MoveUp),
                geng::Key::ArrowLeft => self.model.player_input(Action::MoveLeft),
                geng::Key::ArrowRight => self.model.player_input(Action::MoveRight),
                _ => {}
            },
            geng::Event::MouseRelease { button } => match button {
                geng::MouseButton::Left => {
                    let pos = self
                        .model
                        .camera
                        .screen_to_world(
                            self.framebuffer_size.map(|x| x as f32),
                            self.cursor_pos.map(|x| x as f32),
                        )
                        .map(|x| (x + 0.5).floor() as i64);
                    self.model.player_input(Action::MoveTo(pos));
                }
                geng::MouseButton::Middle => {}
                geng::MouseButton::Right => {}
            },
            geng::Event::TouchEnd(touch) => {
                let pos = self
                    .model
                    .camera
                    .screen_to_world(
                        self.framebuffer_size.map(|x| x as f32),
                        touch.position.map(|x| x as f32),
                    )
                    .map(|x| (x + 0.5).floor() as i64);
                self.model.player_input(Action::MoveTo(pos));
            }
            geng::Event::CursorMove { position } => {
                self.cursor_pos = position;
            }
            _ => {}
        }
        for effect in std::mem::take(&mut self.model.effects) {
            match effect {
                Effect::PlaySound(sound_kind) => {
                    let sound_variants = match sound_kind {
                        SoundKind::TwoSteps => &self.assets.sounds.two_steps,
                        SoundKind::MetalHit => &self.assets.sounds.metal_hit,
                        SoundKind::PourWater => &self.assets.sounds.pour_water,
                        SoundKind::RobotMove => &self.assets.sounds.robot_move,
                    };
                    if let Some(sound) = sound_variants.choose(&mut thread_rng()) {
                        let mut effect = sound.play();
                        effect.set_speed(1.5);
                    }
                }
            }
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.model.update(delta_time);
    }
}
