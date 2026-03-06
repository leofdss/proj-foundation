use tokio::sync::mpsc::Receiver;

use crate::{
    common::entity::AudioRef,
    core::player::{PlayerCommand, PlayerEffect, PlayerEvent, PlayerStore},
    infra::player::engine::PlayerEngine,
};

pub struct PlayerRuntime<E: PlayerEngine> {
    store: PlayerStore,
    engine: E,
    command_rx: Receiver<PlayerCommand>,
    event_rx: Receiver<PlayerEvent>,
}

impl<E: PlayerEngine> PlayerRuntime<E> {
    pub fn new(
        store: PlayerStore,
        engine: E,
        command_rx: Receiver<PlayerCommand>,
        event_rx: Receiver<PlayerEvent>,
    ) -> Self {
        PlayerRuntime {
            store,
            engine,
            command_rx,
            event_rx,
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(cmd) = self.command_rx.recv() => {
                    self.process_command(cmd);
                }
                Some(event) = self.event_rx.recv() => {
                    self.process_command(
                        PlayerCommand::EngineEvent(event)
                    );
                }
            }
        }
    }

    fn process_command(&mut self, command: PlayerCommand) {
        let effects = self.store.dispatch(command);

        for effect in effects {
            self.execute_effect(effect);
        }
    }

    fn execute_effect(&self, effect: PlayerEffect) {
        match effect {
            PlayerEffect::LoadPlaylist => {
                let list: Vec<AudioRef> = self
                    .store
                    .state
                    .tracks
                    .clone()
                    .into_iter()
                    .map(|e| e.audio)
                    .collect();
                self.engine.load_playlist(&list);
            }
            PlayerEffect::StartPlayback => self.engine.start(),
            PlayerEffect::StopPlayback => self.engine.stop(),
        }
    }
}
