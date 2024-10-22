pub struct Settings {
    pub distributor_address: Option<String>,
    pub kafka_address: Option<String>,
    pub kafka_topic: Option<String>,
    pub kafka_partition: Vec<usize>,
    pub mode: Option<String>,
    pub udp_multicast_address: String,
}