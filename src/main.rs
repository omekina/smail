mod config;
mod mail_file;
mod io;
mod connection;
mod sender;
mod flags;


use std::env::args;
use config::loader::{ConfigItem, load_config};
use config::path::{get_home_folder};


fn main() {
    let exit_code = runtime();
    std::process::exit(exit_code);
}


/**
Additional main logic function wrapper for easy exit code returns.
*/
fn runtime() -> i32 {

    /* Initialize the config path (based on user directory). */
    let mut config_path: String = match get_home_folder() {
        Some(home_dir) => home_dir + "/.smailconf",
        None => {
            io::output::error("This tool is developed and tested only for Linux.");
            return 1;
        },
    };

    /* Get console arguments. */
    let mut console_arguments: Vec<String> = args().collect();
    console_arguments.remove(0);

    /* Parse flags from console arguments. */
    let argument_flags = flags::parse_flags(&console_arguments);
    console_arguments.drain(0..argument_flags.len());

    /* Config file flag override. */
    match flags::find_flag("config", &argument_flags) {
        Some(value) => { config_path = value.clone(); },
        None => {},
    };

    /* Override stdin if stdin flag is present. */
    let mut stdin_override = String::new();
    match flags::find_flag("stdin", &argument_flags) {
        Some(value) => { stdin_override = value; },
        None => {},
    };

    /* Get target action and trim the arguments. */
    if console_arguments.len() == 0 {
        io::output::warning("No command specified.");
        return 1;
    }
    let action = console_arguments[0].clone();
    console_arguments.remove(0);

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

    /* If send command was issued -> send e-mail(s). */
    if action == "send" {
        return match sender::send(&console_arguments, &configuration, &stdin_override) {
            Some(_) => 0,
            None => 1,
        };
    }

    /* If no command was matched -> exit. */
    io::output::warning("Command not known:");
    println!("{}", action);
    return 1;
}
