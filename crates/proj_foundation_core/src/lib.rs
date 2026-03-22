use tokio::sync::{broadcast, mpsc};

use crate::{
    api::AppApi, app::App, core::player::PlayerStore, infra::player::LinuxPlayerEngine,
    runtime::PlayerRuntime,
};

pub mod api;
pub mod app;
pub mod common;
pub mod core;
pub mod infra;
pub mod runtime;

async fn linux_bootstrap() {
    let (command_tx, command_rx) = mpsc::channel(32);
    let (event_tx, event_rx) = mpsc::channel(32);
    let (event_broadcast_tx, event_broadcast_rx) = broadcast::channel(32);
    let store = PlayerStore::default();
    let engine = LinuxPlayerEngine::new(event_tx);
    let player = PlayerRuntime::new(store, engine, command_rx, event_rx, event_broadcast_tx);
    let controller = AppApi::new(command_tx, event_broadcast_rx, player);
    let mut app = App::new(controller);
    let _ = app.run();
    let _ = app.api.play();
    let _ = app.api.stop();
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
