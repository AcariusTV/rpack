use std::env;
mod cmenus {
    pub mod help;
}
mod messages {
    pub mod error_message;
    pub mod success_message;
}
mod commands {
    pub mod build;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cmenus::help::help();
    } else {
        match args[1].as_str() {
            "help" => cmenus::help::help(),
            "build" => {
                if args.len() > 2 {
                    commands::build::build(&args[2]);
                } else {
                    messages::error_message::print_error("Not enough arguments were supplied. Please use 'rpack build <filepath>' for building a file.");
                }
            }
             _ => messages::error_message::print_error("Unknown argument. Please use 'help' for an overview of all arguments."),
        }
    }
}
