use crate::{infra::player::engine::PlayerEngine, runtime::PlayerRuntime};

pub struct App<E: PlayerEngine> {
    player: PlayerRuntime<E>,
}

impl<E: PlayerEngine> App<E> {
    pub fn new(player: PlayerRuntime<E>) -> Self {
        App { player }
    }

    pub async fn run(self) {
        self.player.run().await;
    }
}
