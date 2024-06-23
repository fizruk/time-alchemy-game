use crate::{assets::Assets, model::*, render::GameRender};

use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    render: GameRender,
    model: Model,
    framebuffer_size: vec2<usize>,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            render: GameRender::new(geng, assets),
            model: Model::new(),
            framebuffer_size: vec2(1, 1), // dummy
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
            _ => {}
        }
        for effect in std::mem::take(&mut self.model.effects) {
            match effect {
                Effect::PlaySound(sound_kind) => match sound_kind {
                    SoundKind::Steps => {
                        if let Some(sound) = self.assets.sounds.steps.choose(&mut thread_rng()) {
                            let mut effect = sound.play();
                            effect.set_speed(1.5);
                        }
                    }
                },
            }
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.model.update(delta_time);
    }
}
