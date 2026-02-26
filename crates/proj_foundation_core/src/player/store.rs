use super::PlayerCommand;
use super::PlayerEffect;
use super::PlayerState;
use super::player_use_case;

#[derive(Default)]
pub struct PlayerStore {
    state: PlayerState,
}

impl PlayerStore {
    pub fn dispatch(&mut self, command: PlayerCommand) -> Vec<PlayerEffect> {
        let (new_state, effects) = player_use_case(&self.state, command);

        self.state = new_state;

        effects
    }

    pub fn get_state(&self) -> &PlayerState {
        &self.state
    }
}
