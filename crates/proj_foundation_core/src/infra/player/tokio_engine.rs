use crate::common::entity::AudioRef;
use crate::common::units::Milliseconds;
use crate::core::player::PlayerEvent;
use crate::infra::player::engine::PlayerEngine;
use tokio::sync::mpsc::Sender;

pub struct TokioPlayerEngine {
    event_tx: Sender<PlayerEvent>,
}

impl TokioPlayerEngine {
    pub fn new(event_tx: Sender<PlayerEvent>) -> Self {
        TokioPlayerEngine { event_tx }
    }
}

impl PlayerEngine for TokioPlayerEngine {
    fn load_playlist(&self, _playlist: &[AudioRef]) {
        let sender = self.event_tx.clone();

        tokio::spawn(async move {
            // ... playback real ...
            sender.send(PlayerEvent::Ready).await.ok();
        });
    }

    fn start(&self) {
        let sender = self.event_tx.clone();

        tokio::spawn(async move {
            sender.send(PlayerEvent::PlaybackStarted).await.ok();

            sender
                .send(PlayerEvent::PositionUpdated(Milliseconds(1000)))
                .await
                .ok();

            sender
                .send(PlayerEvent::PositionUpdated(Milliseconds(2000)))
                .await
                .ok();

            sender.send(PlayerEvent::PlaybackFinished).await.ok();
        });
    }

    fn stop(&self) {
        let sender = self.event_tx.clone();

        tokio::spawn(async move {
            sender.send(PlayerEvent::PlaybackFinished).await.ok();
        });
    }
}
