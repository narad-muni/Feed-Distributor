use std::sync::{atomic::AtomicBool, Arc};

use bitflags::bitflags;
use crossbeam::queue::SegQueue;
use mio::net::TcpStream;
use tungstenite::WebSocket;

use super::{settings::Mode, work::ClientWork};

#[derive(Debug, Clone)]
pub struct ClientProfile {
    pub conn: Arc<Connection>,
    pub subscriptions: Vec<ClientSubscription>,
    pub mode: Mode,
    pub format: Format,
    pub initialized: bool,
    pub work_list: Arc<SegQueue<ClientWork>>,
    pub work_lock: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Copy)]
pub struct ClientSubscription {
    pub token: usize,
    pub dtype: TypeFlags,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Json,
    Native,
    JsonArray,
}

#[derive(Debug)]
pub enum Connection {
    Ws(WebSocket<TcpStream>),
    Tcp(TcpStream),
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TypeFlags: u8 {
        const DEPTH = 0b00000001;
        const TOUCH_LINE = 0b00000010;
        const MINI_TOUCH_LINE = 0b00000100;

        const ALL = Self::DEPTH.bits() | Self::TOUCH_LINE.bits() | Self::MINI_TOUCH_LINE.bits();
    }
}

impl ClientProfile {
    pub fn create_empty(conn: Arc<Connection>) -> Self {
        Self {
            conn,
            subscriptions: Vec::new(),
            mode: Mode::default(),
            format: Format::Native,
            initialized: false,
            work_list: Arc::new(SegQueue::new()),
            work_lock: Arc::new(AtomicBool::new(false)),
        }
    }
}
