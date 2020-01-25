use std::sync::{Arc, Mutex};
use tokio::sync::watch::Receiver;

pub struct Bug {
    pub receiver: Mutex<Receiver<Arc<u32>>>,
    pub state: Mutex<Arc<u32>>,
}

impl Bug {
    pub async fn run(&self) {
        loop {
            let mut receiver = self.receiver.lock().unwrap();

            if let Some(broadcast) = receiver.recv().await {
                let mut state = self.state.lock().unwrap();
                *state = broadcast;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Bug;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::sync::watch;
    use tokio::time::timeout;

    #[tokio::test]
    async fn run() {
        let (bug_tx, bug_rx) = watch::channel(Arc::new(1));
        bug_tx.broadcast(Arc::new(2)).unwrap();

        let bug = Bug {
            receiver: Mutex::new(bug_rx),
            state: Mutex::new(Arc::new(0)),
        };
        timeout(Duration::from_secs(1), bug.run())
            .await
            .unwrap_err();

        assert_eq!(2, *bug.state.lock().unwrap());
    }
}
