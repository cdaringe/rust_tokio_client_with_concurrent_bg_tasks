use std::sync::{Arc, Mutex};

use tokio::task::JoinHandle;

use crate::refresher_thing::Refresher;

pub struct Client {
    config: Arc<Mutex<u32>>,
    config_listener: JoinHandle<()>,
    config_emitter: JoinHandle<()>,
}

impl Client {
    pub fn new() -> Client {
        let (mut rx, config_emitter) = Refresher::new();
        let config = Arc::new(Mutex::new(0));
        let client_configuration = config.clone();
        let config_listener = tokio::spawn(async move {
            loop {
                let _new_config_available_event = rx.recv().await.unwrap();

                // pretend we've read new config
                let previous_config = *client_configuration.lock().unwrap();
                let next_config = previous_config + 1;
                *client_configuration.lock().unwrap() = next_config;
                println!("next config received: {next_config}");
            }
        });
        Client {
            config_listener,
            config_emitter,
            config,
        }
    }

    pub async fn query(&self) -> u32 {
        let config = { *self.config.clone().lock().unwrap() };
        let query_result = tokio::spawn(async move { config + 100 }).await.unwrap();
        query_result
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.config_listener.abort();
        self.config_emitter.abort();
    }
}
