use crate::{api::AppApi, infra::player::engine::PlayerEngine};

pub struct App<E: PlayerEngine> {
    pub api: AppApi<E>,
}

impl<E: PlayerEngine> App<E> {
    pub fn new(controller: AppApi<E>) -> Self {
        App { api: controller }
    }

    pub async fn run(&mut self) {
        self.api.run().await;
    }
}
