pub mod run_handler;
pub mod undefined_handler;

pub use run_handler::run_handler;
pub use undefined_handler::undefined_handler;

#[macro_export]
macro_rules! nr {
    () => {
        crate::debug_message!("NOT REALIZED!");
    }
}
