use tokio::sync::mpsc;
use crate::domain::order::Order;

pub struct QueueProcessor {
    tx: mpsc::UnboundedSender<Vec<Order>>,
    rx: mpsc::UnboundedReceiver<Vec<Order>>,
}

impl QueueProcessor {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        QueueProcessor { tx, rx }
    }

    pub async fn run(&mut self) {
        println!("QueueProcessor is running");
        while let Some(app_name) = self.rx.recv().await {
            println!("Executing");
        }
    }

    pub fn get_tx(&self) -> mpsc::UnboundedSender<Vec<Order>> {
        self.tx.clone()
    }
}