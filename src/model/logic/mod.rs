use super::*;

impl Model {
    pub fn update(&mut self, _delta_time: Time) {}

    pub fn player_input(&mut self, action: Action) {
        let delta = match action {
            Action::MoveDown => vec2(0, -1),
            Action::MoveUp => vec2(0, 1),
            Action::MoveLeft => vec2(-1, 0),
            Action::MoveRight => vec2(1, 0),
        };
        let target_pos = self.player.pos + delta;
        if target_pos.x >= 0 && target_pos.x < self.level_map.size.x {
            if target_pos.y >= 0 && target_pos.y < self.level_map.size.y {
                self.player.pos = target_pos;
            }
        }
    }
}
