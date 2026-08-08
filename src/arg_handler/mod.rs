mod handlers;

use paste::paste;

// macro_rules! skip_macro_item {
//     ($expr:expr, $smth:tt) => {$expr}
// } 

macro_rules! ArgCommands {
    ($($command_name:ident $({$($ele_name:ident:$ele_type:ty),*})?),*) => {
        #[derive(Debug)]
        pub enum ArgCommands {
            $($command_name $({$($ele_name:$ele_type),*})?),*
        }

        impl ArgCommands {
            pub async fn handle(args: Vec<String>) {
                match ArgCommands::parse(args) {
                    $(
                        ArgCommands::$command_name$({$($ele_name),*})? => {
                            paste!(handlers::[<$command_name:lower _handler>])($($($ele_name),*),*).await;
                        }
                    )*
                }
            
            }

            fn parse(args: Vec<String>) -> ArgCommands {
                let mut iter = args.into_iter();
                while let Some(arg) = iter.next() {
                    match arg.as_str() {
                        $(
                            paste!(stringify!([<$command_name:lower>])) => {
                                return ArgCommands::$command_name $({$($ele_name:iter.next()),*})?
                            }
                        )*
                        _ => {}
                    }
                }
                ArgCommands::Undefined
            }
        }
    };
}

ArgCommands! {
    Run         {flag: Option<String>, config: Option<String>},
    Undefined 
}

#[derive(Debug)]
struct ArgCommandsError {
    error_message: String
}

impl ArgCommandsError {
    fn new(error_message: &'static str) -> Self{
        ArgCommandsError { error_message: error_message.to_string() }
    }
}

impl std::fmt::Display for ArgCommandsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "{}", self.error_message); // VS CODE PROBLEM!
        res
    }
}

impl std::error::Error for ArgCommandsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
