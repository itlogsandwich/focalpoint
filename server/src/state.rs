use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct RoomState 
{
    pub tx: broadcast::Sender<String>,
}

#[derive(Clone)]
pub struct AppState 
{
    pub rooms: Arc<DashMap<String, RoomState>>
}

impl AppState
{
    pub fn new() -> Self 
    {
        Self 
        {
            rooms: Arc::new(DashMap::new())
        }
    }
}
