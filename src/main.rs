use input::client_input::ClientInput;

mod constants;
mod globals;
mod input;
mod macros;
mod output;
mod threadpool;
mod types;
mod utils;

fn main() {
    globals::init();
    let mut client_input = ClientInput::new();

    client_input.start_input();
}
