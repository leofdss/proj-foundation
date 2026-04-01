use tokio::sync::{broadcast, mpsc};

use crate::{
    api::AudioRefDto,
    common::entity::{AudioRef, AudioRefId},
    core::player::{PlayerCommand, PlayerEvent},
    infra::player::PlayerEngine,
    runtime::PlayerRuntime,
};

pub struct AppApi<E: PlayerEngine> {
    command_tx: mpsc::Sender<PlayerCommand>,
    event_broadcast_rx: broadcast::Receiver<PlayerEvent>,
    player: PlayerRuntime<E>,
}

impl<E: PlayerEngine> AppApi<E> {
    pub fn new(
        command_tx: mpsc::Sender<PlayerCommand>,
        event_broadcast_rx: broadcast::Receiver<PlayerEvent>,
        player: PlayerRuntime<E>,
    ) -> Self {
        AppApi {
            command_tx,
            event_broadcast_rx,
            player,
        }
    }

    pub async fn run(&mut self) {
        self.player.run().await;
    }
}

impl<E: PlayerEngine> AppApi<E> {
    pub fn play(&self) {
        let command_tx = self.command_tx.clone();
        tokio::spawn(async move {
            command_tx.send(PlayerCommand::Play).await.ok();
        });
    }

    pub fn stop(&self) {
        let command_tx = self.command_tx.clone();
        tokio::spawn(async move {
            command_tx.send(PlayerCommand::Stop).await.ok();
        });
    }

    pub fn load_playlist(&self, audios_dto: Vec<AudioRefDto>) {
        let audios = self.audios_ref_converter(audios_dto);
        let command_tx = self.command_tx.clone();
        tokio::spawn(async move {
            command_tx
                .send(PlayerCommand::LoadPlaylist(audios))
                .await
                .ok();
        });
    }

    pub fn subscribe_events(&self) -> broadcast::Receiver<PlayerEvent> {
        self.event_broadcast_rx.resubscribe().recv()
    }

    fn audios_ref_converter(&self, audios: Vec<AudioRefDto>) -> Vec<AudioRef> {
        audios
            .into_iter()
            .map(|audio| self.audio_ref_converter(audio))
            .collect()
    }

    fn audio_ref_converter(&self, audio: AudioRefDto) -> AudioRef {
        AudioRef {
            id: AudioRefId(audio.id.0),
            path: audio.path,
        }
    }
}
