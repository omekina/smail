use std::{fs::write, path::Path};
use crate::io;


const DEFAULT_CONFIG: &str = "# ===== GENERATED WITH 'smail init' =====

# Welcome to SMAIL configuration file.
# The following is a set of values to be used in secure e-mail_file connection.


# Host is the smtp hostname of the e-mail_file server
host={host}
# TLS/SSL port for the ESMTP connection
port=587


# Username for authentication
username={username}
# Password for authentication
password={password}


# The e-mail_file file created with 'smail create' command will be placed into this template.
# Use {body} and {style} to insert the content.
# Tabs are here just for configuration file clarity.
# Since the insertion blocks are directly replaced with the text from e-mail_file files,
# the tabbing will not be preserved.
BEGIN TEMPLATE
<!DOCTYPE html>
<html>
        <head>
                <meta http-equiv=\"content-type\" content=\"text/html; charset=UTF-8\">
                <style>
                        {STYLE}
                </style>
        </head>
        <body>
                {BODY}
        </body>
</html>
";


/**
Find values from `needle` in `haystack` and replace them with values in `replacement` respectively.

WARNING: This function is not made for use outside of this module and should be optimized
if outside use is needed.
 */
fn find_and_replace(needle: Vec<&str>, replacement: Vec<&String>, haystack: &str) -> Option<String> {
    if needle.len() != replacement.len() {
        return None;
    }
    let mut result = String::from(haystack);
    for search_index in 0..needle.len() {
        result = result.replace(
            needle.get(search_index).unwrap(),
            replacement.get(search_index).unwrap()
        );
    }
    return Some(result);
}


/**
Fetch the template and replace provided fields.
*/
fn generate_config_file(
    host: &String,
    username: &String,
    password: &String,
) -> String {
    let result = find_and_replace(
        vec!["{host}", "{username}", "{password}"],
        vec![host, username, password],
        DEFAULT_CONFIG,
    ).unwrap();
    return result;
}


/**
Create `~/.smailconf` configuration file with interactive prompt.
*/
pub fn init(filepath: &String, arguments: &Vec<String>) -> bool {

    // Find overwrite flag in console arguments
    let mut overwrite = false;
    for argument in arguments {
        if argument == "--overwrite" {
            overwrite = true;
            break;
        }
    }

    // Check if config file exists and if overwrite is requested
    let config_file_exists = Path::new(filepath).exists();
    if config_file_exists & ! overwrite {
        io::output::warning("Config file already exists and overwrite was not requested.");
        return false;
    }

    // Get information from user
    let user_input = match io::input::read_items(
        vec!["SMTP server hostname", "Your username", "Your password"]
    ) {
        Some(value) => value,
        None => {
            io::output::error("Error when reading user input from console.");
            return false;
        }
    };

    // Generate config file from template
    let generated_content = generate_config_file(
        user_input.get(0).unwrap(),
        user_input.get(1).unwrap(),
        user_input.get(2).unwrap(),
    );

    return match write(filepath, generated_content) {
        Ok(_) => {
            io::output::success("Written config file.");
            true
        },
        Err(_) => {
            io::output::error("Could not write config file.");
            false
        }
    };
}
