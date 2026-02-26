pub mod app;
pub mod core;
pub mod player;

use app::App;
use player::PlayerEffectHandler;
use player::PlayerEngineLinux;
use player::PlayerStore;

fn main() {
    let store = PlayerStore::default();
    let engine = PlayerEngineLinux::new();
    engine.run();

    let effect_handler = PlayerEffectHandler::new(engine);
    let app = App::new(store, effect_handler);
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
