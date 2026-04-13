use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, Clone)]
pub struct ProgressEvent {
    pub msg: String,
    pub done: bool,
    pub error: Option<String>,
}

pub struct DownloadState {
    pub pid: Arc<Mutex<Option<u32>>>,
    pub cancelled: Arc<Mutex<bool>>,
}
