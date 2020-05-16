use tokio::sync::mpsc;

pub type Result<T> = anyhow::Result<T>;
pub type Sender<T> = mpsc::Sender<T>;
pub type Receiver<T> = mpsc::Receiver<T>;
