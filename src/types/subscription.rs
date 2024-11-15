use super::client_profile::ClientProfile;

pub struct Subscription<'a> {
    pub all_clients: Vec<&'a ClientProfile>,
    pub tcp_clients: Vec<&'a ClientProfile>,
    total_udp_count: usize,
    total_count: usize,
    udp_type_count: TypeCount,
    tcp_type_count: TypeCount,
}

pub struct TypeCount {
    pub depth_count: usize,
    pub touch_line_count: usize,
    pub mini_touch_line_count: usize,
}
