use std::sync::{Arc, Mutex};
use crate::ws::WsChannels;
use crate::state::AppState;

#[derive(Clone)]
pub struct CombinedState {
    pub chess: Arc<Mutex<AppState>>,
    pub ws: WsChannels,
}

impl CombinedState {
    pub fn new() -> Self {
        Self {
            chess: Arc::new(Mutex::new(AppState::new())),
            ws: WsChannels::new(),
        }
    }
}
