/*      Programm messages    */
#[macro_export]
macro_rules! info_message{
    ($expr:expr) => {
        println!("{} {}", format!("[INFO]  ").yellow(), $expr)
    }
}

#[macro_export]
macro_rules! error_message{
    ($expr:expr) => {
        println!("{} {}", format!("[ERROR] ").red(), $expr)
    }
}

#[macro_export]
macro_rules! debug_message{
    ($expr:expr) => {
        println!("{} {}", format!("[DEBUG] ").purple(), stringify!($expr))
    }
}

/*      XRAY messages    */
#[macro_export]
macro_rules! command_message {
    ($command_name:ident, $message:expr) => {
        println!("{} {}", format!("[{}]", stringify!($command_name)).green(), $message)
    }
}