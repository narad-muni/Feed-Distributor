use crate::threadpool::WorkTrait;

use super::client_profile::ClientProfile;

#[derive(Debug, Clone, Copy)]
pub struct ClientWork {
    pub work_type: WorkType,
    pub client_profile: &'static ClientProfile,
}

impl WorkTrait for ClientWork {
    fn do_work(&self) {}
}

#[derive(Debug, Clone, Copy)]
pub struct FeedWork {
    pub work_type: WorkType,
    pub processing_fn: fn(&Self),
}

impl WorkTrait for FeedWork {
    fn do_work(&self) {
        (self.processing_fn)(self);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WorkType {
    TokenWiseLatest(usize),
    TokenWise(usize),
    MarketMessage,
}
