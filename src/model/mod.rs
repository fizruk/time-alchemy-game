mod logic;

use geng::prelude::*;

pub type Time = R32;

pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
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
    Sword { damage: DP, cost: Coins },
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
}

pub struct Model {
    pub camera: Camera2d,
    pub player: Player,
    pub level_map: LevelMap,
}

impl Model {
    pub fn new() -> Self {
        Self {
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
                        kind: ItemKind::Sword {
                            damage: 3,
                            cost: 10,
                        },
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
            },
        }
    }
}
