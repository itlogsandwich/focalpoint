use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::broadcast;

pub struct RoomState 
{
    pub tx: broadcast::Sender<String>,
}

pub struct AppState 
{
    pub rooms: DashMap<String, RoomState>
}

impl AppState
{
    pub fn new() -> Self 
    {
        Self 
        {
            rooms: DashMap::new();
        }
    }
}
