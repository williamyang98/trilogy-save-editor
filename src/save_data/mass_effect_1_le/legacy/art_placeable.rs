use serde::{Deserialize, Serialize};

use crate::save_data::Dummy;

use super::BaseObject;

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct ArtPlaceableBehavior {
    is_dead: bool,
    generated_treasure: bool,
    challenge_scaled: bool,
    owner: Option<BaseObject>,
    health: f32,
    health: f32,
    enabled: bool,
    current_fsm_state_name: String,
    is_destroyed: bool,
    state_0: String,
    state_1: String,
    use_case: u8,
    use_case_override: bool,
    player_only: bool,
    skill_difficulty: u8,
    inventory: Option<BaseObject>,
    skill_game_failed: bool,
    skill_game_xp_awarded: bool,
}

#[rcize_fields]
#[derive(Deserialize, Serialize, Clone, RawUiChildren)]
pub struct ArtPlaceable {
    _unknown: Dummy<60>,
}
