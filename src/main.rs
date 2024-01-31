pub mod commands;
pub mod custom_error;
pub mod fan_variants;

use std::env::args;

fn is_debug_flag(args: &mut Vec<String>) -> bool {
    if let Some(index) = args.iter().position(|arg| arg == "-d" || arg == "--debug") {
        args.remove(index);
        return true;
    }

    return false;
}

fn main() {
    let mut argv: Vec<String> = args().collect();
    let debug_mode = is_debug_flag(&mut argv);
    let command = commands::command_parser(&argv[1..]);

    let command_output = match command {
        Ok(command) => command.execute(),
        Err(err) => {
            if debug_mode {
                println!("{:?}", err);
            } else {
                println!("{}", err);
            }
            return;
        }
    };

    let output = match command_output {
        Ok(output) => output,
        Err(err) => {
            if debug_mode {
                format!("{:?}", err)
            } else {
                format!("{}", err)
            }
        }
    };

    println!("{}", output);
}
