use tokio::{
    sync::mpsc::{channel, Receiver},
    task::JoinHandle,
};
pub struct Refresher {}

impl Refresher {
    pub fn new() -> (Receiver<()>, JoinHandle<()>) {
        let (tx, rx) = channel(1);
        let event_emitter = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                println!("emitting event from watcher/refresher");
                let _ = tx.send(()).await;
            }
        });
        (rx, event_emitter)
    }
}
