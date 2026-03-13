use tokio::sync::mpsc;

use crate::{
    app::App, core::player::PlayerStore, infra::player::LinuxPlayerEngine, runtime::PlayerRuntime,
};

pub mod app;
pub mod common;
pub mod core;
pub mod infra;
pub mod runtime;

async fn linux_bootstrap() {
    let (command_tx, command_rx) = mpsc::channel(32);
    let (event_tx, event_rx) = mpsc::channel(32);
    let store = PlayerStore::default();
    let engine = LinuxPlayerEngine::new(event_tx);
    let player = PlayerRuntime::new(store, engine, command_rx, event_rx);
    let app = App::new(player);
    app.run().await;
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
