use crate::core::player::{
    PlayerCommand, PlayerEffectHandler, PlayerEngine, PlayerState, PlayerStore,
};

pub struct PlayerRuntime<E: PlayerEngine> {
    store: PlayerStore,
    effect_handler: PlayerEffectHandler<E>,
}

impl<E: PlayerEngine> PlayerRuntime<E> {
    pub fn new(store: PlayerStore, effect_handler: PlayerEffectHandler<E>) -> Self {
        PlayerRuntime {
            store,
            effect_handler,
        }
    }

    pub fn run(&self) {
        println!("run");
    }

    pub fn dispatch(&mut self, command: PlayerCommand) {
        let mut queue = vec![command];

        while let Some(cmd) = queue.pop() {
            let effects = self.store.dispatch(cmd);
            let new_commands = self.effect_handler.handle(effects, &self.store.state);
            queue.extend(new_commands);
        }
    }

    pub fn state(&self) -> &PlayerState {
        self.store.get_state()
    }
}
