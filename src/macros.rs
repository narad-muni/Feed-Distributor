#[macro_export]
// Macro to create constant arrays
macro_rules! create_array {
    ($constructor:expr; $count:expr) => {{
        [const { $constructor }; $count]
    }};
}
