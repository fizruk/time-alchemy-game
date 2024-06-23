mod logic;

use geng::prelude::*;
use log::Level;

pub type Time = R32;

pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveTo(vec2<i64>),
}

pub struct Backpack {
    pub ingredients: Vec<Ingredient>,
    pub coins: Coins,
}

pub struct Player {
    pub pos: vec2<i64>,
    pub health: HP,
    pub backpack: Backpack,
}

type HP = i64;

type DP = i64;

type Coins = i64;

pub enum Ingredient {
    Leaf,
    Ice,
    Branch,
    Banana,
    Cherry,
    Blueberry,
    Strawberry,
}

pub enum ItemKind {
    Sword { damage: DP },
    Ingredient(Ingredient),
}

pub struct Item {
    pub pos: vec2<i64>,
    pub kind: ItemKind,
}

pub struct Enemy {
    pub pos: vec2<i64>,
    pub health: HP,
    pub damage: DP,
}

pub struct LevelMap {
    pub size: vec2<i64>,
    pub items: Vec<Item>,
    pub enemies: Vec<Enemy>,
    pub expansion_cells: Vec<vec2<i64>>,
}

impl LevelMap {
    pub fn inside(&self, pos: vec2<i64>) -> bool {
        let inside_original =
            pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y;
        inside_original || self.expansion_cells.contains(&pos)
    }

    pub fn adjacent(&self, pos: vec2<i64>) -> bool {
        let inside_original_expansion =
            pos.x >= -1 && pos.x <= self.size.x && pos.y >= 0 && pos.y < self.size.y
                || pos.x >= 0 && pos.x < self.size.x && pos.y >= -1 && pos.y <= self.size.y;
        !self.inside(pos)
            && (inside_original_expansion
                || self
                    .expansion_cells
                    .iter()
                    .any(|cell| manhattan_dist(*cell, pos) == 1))
    }

    pub fn cells_iter(&self) -> impl Iterator<Item = vec2<i64>> + '_ {
        (0..self.size.x)
            .flat_map(|x| (0..self.size.y).map(move |y| vec2(x, y)))
            .chain(self.expansion_cells.iter().copied())
    }
}

pub fn manhattan_dist(pos1: vec2<i64>, pos2: vec2<i64>) -> i64 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

pub enum SoundKind {
    Steps,
}

pub enum Effect {
    PlaySound(SoundKind),
}

pub struct Model {
    pub camera: Camera2d,
    pub player: Player,
    pub level_map: LevelMap,
    pub effects: Vec<Effect>,
    pub state: State,
}

pub enum State {
    Day,
    ExpandMap,
    Shop,
    Night,
}

impl Model {
    pub fn new() -> Self {
        let mut model = Self {
            camera: Camera2d {
                center: vec2(0.0, 0.0),
                rotation: Angle::ZERO,
                fov: 10.0,
            },
            player: Player {
                pos: vec2(2, 1),
                health: 100,
                backpack: Backpack {
                    ingredients: vec![],
                    coins: 10,
                },
            },
            level_map: LevelMap {
                size: vec2(3, 3),
                items: vec![
                    Item {
                        pos: vec2(2, 0),
                        kind: ItemKind::Sword { damage: 2 },
                    },
                    Item {
                        pos: vec2(1, 2),
                        kind: ItemKind::Ingredient(Ingredient::Leaf),
                    },
                ],
                enemies: vec![Enemy {
                    pos: vec2(0, 1),
                    health: 3,
                    damage: 0,
                }],
                expansion_cells: vec![],
            },
            effects: vec![],
            state: State::Day,
        };
        model.camera.center = model.player.pos.map(|x| x as f32);
        model
    }
}
