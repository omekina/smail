use std::fs::read_to_string;
use crate::io::output;


pub struct ConfigItem {
    /**
    Example: username, password, host, port, ...

    Key can also be '=' in this case the value is the html template for e-mails.
    Since '=' is reserved as a delimiter for key-value pair, it can be easily stored such that
    there are no collisions.
     */
    pub key: String,
    pub value: String,
}


/**
Load configuration file at specified filepath into key-value pairs.
*/
pub fn load_config(config_file_path: &String) -> Option<Vec<ConfigItem>> {
    let contents: String = match read_to_string(config_file_path) {
        Ok(value) => value,
        Err(_) => {
            output::error("Could not read from configuration file");
            println!("Maybe you forgot to 'smail init'?");
            return None;
        },
    };

    return parse_config(&contents);
}


/**
Main configuration parsing function.
*/
pub fn parse_config(config_file_contents: &String) -> Option<Vec<ConfigItem>> {

    let config_file_lines = config_file_contents.split('\n');
    let mut parsed_config: Vec<ConfigItem> = Vec::new();

    // For storing template at the end of config file
    let mut temp: String = String::new();

    // If the line pointer is already inside the template section
    let mut is_template_section: bool = false;

    for current_line in config_file_lines {

        // Template section
        if is_template_section {
            temp += current_line;
            temp += "\n";
            continue;
        }

        // Template section delimiter
        if current_line == "BEGIN TEMPLATE" {
            is_template_section = true;
            continue;
        }

        // Ignore blank lines
        if current_line == "" {
            continue;
        }

        // Ignore comment lines
        if match current_line.chars().nth(0) {
            Some(letter) => letter,
            None => { continue; }
        } == '#' {
            continue;
        }

        // Separate line by equal sign and save key-value pair
        let line_split: Vec<&str> = current_line.splitn(2, "=").collect();
        if line_split.len() != 2 {
            output::error("Bad config file syntax at line:");
            println!("{}", current_line);
            return None;
        }
        parsed_config.push(ConfigItem {
            key: String::from(*line_split.get(0)?),
            value: String::from(*line_split.get(1)?),
        });
    }

    // Save template section as value with reserved key '='
    parsed_config.push(ConfigItem {
        key: String::from("="),
        value: temp,
    });

    return Some(parsed_config);
}


/**
Search given array of ConfigItems and return value with specified key.
*/
pub fn search_key_in_config(configuration: &Vec<ConfigItem>, key: &str) -> Option<String> {
    for current_item in configuration {
        if current_item.key == key {
            return Some(current_item.value.clone());
        }
    }
    output::error("Configuration file entry was not found:");
    println!("{}", key);
    return None;
}
