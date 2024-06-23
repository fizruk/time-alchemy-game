use geng::prelude::*;

#[derive(geng::asset::Load)]
pub struct Assets {
    pub sprites: Sprites,
}

#[derive(geng::asset::Load)]
pub struct Sprites {
    pub player: ugli::Texture,
    pub enemy: ugli::Texture,
    pub sword: ugli::Texture,
    pub leaf: ugli::Texture,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}
