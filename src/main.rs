mod config;
mod mail_file;
mod io;
mod connection;


use std::env::args;
use config::loader::{ConfigItem, load_config, get_home_folder};


fn main() {
    let exit_code = runtime();
    std::process::exit(exit_code);
}


/**
Additional main logic function wrapper for easy exit code returns.
*/
fn runtime() -> i32 {

    /* Initialize the config path (based on user directory). */
    let config_path: String = match get_home_folder() {
        Some(home_dir) => home_dir + "/.smailconf",
        None => {
            io::output::error("This tool is developed and tested only for Linux.");
            return 1;
        },
    };

    /* Get arguments from console. */
    let console_arguments: Vec<String> = args().collect();
    if console_arguments.len() < 2 {
        io::output::warning("No command specified.");
        return 1;
    }
    let action = console_arguments.get(1).unwrap();

    /* If init command was issued -> make config and do not continue. */
    if action == "init" {
        return match config::maker::init(&config_path, &console_arguments) {
            true => 0,
            false => 1,
        };
    }

    /* If create command was issued -> make e-mail_file files and do not continue. */
    if action == "create" {
        return match mail_file::maker::create(&console_arguments) {
            true => 0,
            false => 1,
        };
    }

    /* Read and parse config file. */
    let configuration: Vec<ConfigItem> = match load_config(&config_path) {
        Some(parsed_config) => parsed_config,
        None => return 1,
    };

    /* If no command was matched -> exit. */
    io::output::warning("Command not known:");
    println!("{}", action);
    return 1;
}
