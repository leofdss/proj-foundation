use crate::{core::player::PlayerEngine, runtime::PlayerRuntime};

pub struct App<E: PlayerEngine> {
    player: PlayerRuntime<E>,
}

impl<E: PlayerEngine> App<E> {
    pub fn new(player: PlayerRuntime<E>) -> Self {
        App { player }
    }

    pub fn run(&self) {
        self.player.run();
    }
}
