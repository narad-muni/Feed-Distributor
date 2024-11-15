use crate::{
    create_array,
    types::{
        client_profile::ClientProfile,
        keep_latest::KeepLatest,
        packet::OutputPacket,
        reuse_array::ReuseArr,
        settings::{self, Mode, Settings},
    },
};
use crossbeam::queue::SegQueue;
use lazy_static::lazy_static;
use std::sync::OnceLock;

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();
pub static DATA_STORE: [KeepLatest<usize>; 35000] = create_array!(KeepLatest::new(); 35000);
pub static TOKEN_PACKETS_QUEUE: [SegQueue<OutputPacket>; 35000] = create_array!(SegQueue::new(); 35000);
pub static MARKET_MESSAGES_QUEUE: SegQueue<OutputPacket> = SegQueue::new();

lazy_static! {
    pub static ref MODE: Mode = settings::get().mode.clone();
    pub static ref CLIENTS_LIST: ReuseArr<ClientProfile> = ReuseArr::new();
}

pub fn init() {
    let args = std::env::args().collect::<Vec<String>>();
    let settings_path = args.get(1).expect("Settings path not provided");

    // Skip first 2 index
    CLIENTS_LIST.reserve();
    CLIENTS_LIST.reserve();

    settings::init(settings_path);
}
