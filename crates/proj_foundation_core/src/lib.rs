pub mod app;
pub mod common;
pub mod core;
pub mod infra;
pub mod runtime;
use crate::{
    app::App,
    core::player::{PlayerEffectHandler, PlayerStore},
    infra::player::engine_linux::PlayerEngineLinux,
    runtime::PlayerRuntime,
};

pub fn player_coordinator_linux_factory() -> PlayerRuntime<PlayerEngineLinux> {
    let store = PlayerStore::default();
    let engine = PlayerEngineLinux::new();

    let effect_handler = PlayerEffectHandler::new(engine);
    let coordinator = PlayerRuntime::new(store, effect_handler);

    coordinator
}

fn main() {
    let player = player_coordinator_linux_factory();
    let app = App::new(player);
    app.run();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
