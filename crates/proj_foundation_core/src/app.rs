use crate::player::PlayerCommand;
use crate::player::PlayerEffectHandler;
use crate::player::PlayerEngine;
use crate::player::PlayerState;
use crate::player::PlayerStore;

pub struct App<E: PlayerEngine> {
    store: PlayerStore,
    effect_handler: PlayerEffectHandler<E>,
}

impl<E: PlayerEngine> App<E> {
    pub fn new(store: PlayerStore, effect_handler: PlayerEffectHandler<E>) -> Self {
        App {
            store,
            effect_handler,
        }
    }

    pub fn run(&self) {
        println!("run");
    }

    pub fn dispatch(&mut self, command: PlayerCommand) {
        let effects = self.store.dispatch(command);
        self.effect_handler.handle(effects);
    }

    pub fn state(&self) -> &PlayerState {
        self.store.get_state()
    }
}
