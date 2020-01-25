use library::Bug;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (bug_tx, bug_rx) = watch::channel(Arc::new(0));

    let bug = Bug {
        receiver: Mutex::new(bug_rx),
        state: Mutex::new(Arc::new(1)),
    };

    tokio::spawn(async move { bug.run().await });
}
