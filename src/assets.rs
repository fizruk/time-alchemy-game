use geng::prelude::*;

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
