use geng::prelude::*;
use geng_utils::gif::GifFrame;

#[derive(geng::asset::Load)]
pub struct Assets {
    pub sprites: Sprites,
    pub sounds: Sounds,
}

#[derive(geng::asset::Load)]
pub struct Sprites {
    pub player: ugli::Texture,
    pub enemy: ugli::Texture,
    pub sword: ugli::Texture,
    pub leaf: ugli::Texture,
    #[load(load_with = "load_gif(&manager, &base_path.join(\"level_1_bot_idle_normal.gif\"))")]
    pub level_1_bot_idle_normal: Vec<GifFrame>,
    #[load(load_with = "load_gif(&manager, &base_path.join(\"level_1_bot_idle_damaged.gif\"))")]
    pub level_1_bot_idle_damaged: Vec<GifFrame>,
    #[load(load_with = "load_gif(&manager, &base_path.join(\"level_1_bot_die.gif\"))")]
    pub level_1_bot_die: Vec<GifFrame>,
}

#[derive(geng::asset::Load)]
pub struct Sounds {
    #[load(list = "1..=6", path = "two-steps/two-steps-*.wav")]
    pub two_steps: Vec<geng::Sound>,
    #[load(list = "1..=2", path = "pour-water/pour-water-*.wav")]
    pub pour_water: Vec<geng::Sound>,
    #[load(list = "1..=3", path = "metal-hit/metal-hit-*.wav")]
    pub metal_hit: Vec<geng::Sound>,
    #[load(list = "1..=2", path = "robot-move/robot-move-*.wav")]
    pub robot_move: Vec<geng::Sound>,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}

fn load_gif(
    manager: &geng::asset::Manager,
    path: &std::path::Path,
) -> geng::asset::Future<Vec<GifFrame>> {
    let manager = manager.clone();
    let path = path.to_owned();
    async move {
        geng_utils::gif::load_gif(
            &manager,
            &path,
            geng_utils::gif::GifOptions {
                frame: geng::asset::TextureOptions {
                    filter: ugli::Filter::Nearest,
                    ..Default::default()
                },
            },
        )
        .await
    }
    .boxed_local()
}
