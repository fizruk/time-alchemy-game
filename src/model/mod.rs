mod logic;

use geng::prelude::*;
use itertools::Itertools;
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
    pub turns_remaining: usize,
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
    pub state: EnemyState,
    pub mode: EnemyMode,
    pub animation_clock: Time,
}

impl Enemy {
    pub fn new(pos: vec2<i64>) -> Self {
        Self {
            pos,
            health: 3,
            damage: 0,
            state: EnemyState::Action(Cooldown::new(EnemyAction::Spawn, r32(0.5))),
            mode: EnemyMode::Normal,
            animation_clock: r32(0.0),
        }
    }

    pub fn take_damage(&mut self, damage: DP) {
        self.health -= damage;
        if self.health > 0 {
            self.state = EnemyState::Action(Cooldown::new(EnemyAction::TakeDamage, r32(0.5)));
        } else {
            self.state = EnemyState::Action(Cooldown::new(EnemyAction::Die, r32(0.5)));
        }
        self.animation_clock = r32(0.0);
        self.mode = EnemyMode::Damaged;
    }
}

pub enum EnemyState {
    Idle,
    Action(Cooldown<EnemyAction>),
}

pub struct Cooldown<T> {
    pub action: T,
    pub leftover: Time,
    pub total: Time,
}

impl<T> Cooldown<T> {
    pub fn new(action: T, total: Time) -> Self {
        Self {
            action,
            total,
            leftover: total,
        }
    }
}

pub enum EnemyMode {
    Normal,
    Damaged,
}

pub enum EnemyAction {
    TakeDamage,
    Attack,
    Die,
    Spawn,
}

pub struct LevelMap {
    pub size: vec2<i64>,
    pub items: Vec<Item>,
    pub enemies: Vec<Enemy>,
    pub dead_enemies: Vec<Enemy>,
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

    pub fn adjacent_cells_iter(&self) -> impl Iterator<Item = vec2<i64>> + '_ {
        (-1..=self.size.x)
            .flat_map(|x| (-1..=self.size.y).map(move |y| vec2(x, y)))
            .chain(self.expansion_cells.iter().flat_map(|cell| {
                (-1..=1).flat_map(move |dx| {
                    (-1..=1)
                        .filter(move |dy| manhattan_dist(vec2(0, 0), vec2(dx, *dy)) == 1)
                        .map(move |dy| *cell + vec2(dx, dy))
                })
            }))
            .unique()
            .filter(|cell| self.adjacent(*cell))
    }
}

pub fn manhattan_dist(pos1: vec2<i64>, pos2: vec2<i64>) -> i64 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()
}

pub enum SoundKind {
    TwoSteps,
    MetalHit,
    PourWater,
    RobotMove,
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
                turns_remaining: 3,
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
                enemies: vec![Enemy::new(vec2(0, 1))],
                dead_enemies: vec![],
                expansion_cells: vec![],
            },
            effects: vec![],
            state: State::Day,
        };
        model.camera.center = model.player.pos.map(|x| x as f32);
        model
    }
}
