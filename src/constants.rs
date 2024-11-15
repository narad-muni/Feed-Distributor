use mio::Token;

pub const OUTPUT_BUF_SIZE: usize = 1024;
pub const INPUT_BUF_SIZE: usize = 1024;
pub const TCP_LISTENER_TOKEN: Token = Token(0);
pub const WS_LISTENER_TOKEN: Token = Token(1);
pub const EVENT_CAPACITY: usize = 128;
