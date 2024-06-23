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
    #[load(load_with = "load_gif(&manager, &base_path.join(\"level_1_bot_idle.gif\"))")]
    pub level_1_bot_idle: Vec<GifFrame>,
}

#[derive(geng::asset::Load)]
pub struct Sounds {
    #[load(list = "1..=6", path = "two-steps/two-steps-*.wav")]
    pub steps: Vec<geng::Sound>,
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
