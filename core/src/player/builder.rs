use std::sync::Arc;

use failure::Error;

use crate::player::{Player, PlayerBackend, PlayerBus, PlayerQueue};
use crate::Rustic;

pub struct PlayerBuilder {
    core: Arc<Rustic>,
    pub name: Option<String>,
    backend: Option<Box<dyn PlayerBackend>>,
    queue: Option<Box<dyn PlayerQueue>>,
    bus: PlayerBus,
}

impl PlayerBuilder {
    pub fn new(core: Arc<Rustic>) -> Self {
        let bus = PlayerBus::new();

        PlayerBuilder {
            core,
            name: None,
            backend: None,
            queue: None,
            bus,
        }
    }

    pub fn with_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_player<P>(&mut self, builder: P) -> Result<&mut Self, Error>
    where
        P: FnOnce(Arc<Rustic>, PlayerBus) -> Result<Box<dyn PlayerBackend>, Error>,
    {
        let backend = builder(Arc::clone(&self.core), self.bus.clone())?;
        self.backend = Some(backend);

        Ok(self)
    }

    pub fn with_queue<Q>(&mut self, builder: Q) -> Result<&mut Self, Error>
    where
        Q: Fn(Arc<Rustic>, PlayerBus) -> Result<Box<dyn PlayerQueue>, Error>,
    {
        let queue = builder(Arc::clone(&self.core), self.bus.clone())?;
        self.queue = Some(queue);

        Ok(self)
    }

    pub fn build(&mut self) -> Arc<Player> {
        assert!(self.backend.is_some());
        assert!(self.queue.is_some());
        assert!(self.name.is_some());
        let backend = self.backend.take().unwrap();
        let queue = self.queue.take().unwrap();
        let name = self.name.take().unwrap();

        Player::new(
            name,
            backend,
            queue,
            self.bus.clone(),
            Arc::clone(&self.core),
        )
    }
}
