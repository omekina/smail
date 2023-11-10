use crate::config;
use crate::config::loader::ConfigItem;
use crate::io;
use crate::mail_file;



/* Config file tests */



const CONFIG_MAKE_EXPECTED_CONTENT: &str = "# ===== GENERATED WITH 'smail init' =====

# Welcome to SMAIL configuration file.
# The following is a set of values to be used in secure e-mail_file connection.


# Host is the smtp hostname of the e-mail_file server
host=test.nonexistent.domain
# TLS/SSL port for the ESMTP connection
port=587


# Username for authentication
username=alice@nonexistent.domain
# Password for authentication
password=very secret


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

#[test]
fn config_generation() {
    println!("Testing config generation");
    let generated_contents = config::maker::generate_config_file(
        &String::from("test.nonexistent.domain"),
        &String::from("alice@nonexistent.domain"),
        &String::from("very secret"),
    );
    assert_eq!(generated_contents, CONFIG_MAKE_EXPECTED_CONTENT);
}


const CONFIG_PARSE_TO_PARSE: &str = "# this should not be included

host=test.nonexistent.domain
port=587
BEGIN TEMPLATE
Hello, tests!
";

fn config_parse_expected_result() -> Vec<ConfigItem> {
    return vec![
        ConfigItem {
            key: String::from("host"),
            value: String::from("test.nonexistent.domain"),
        },
        ConfigItem {
            key: String::from("port"),
            value: String::from("587"),
        },
        ConfigItem {
            key: String::from("="),
            value: String::from("Hello, tests!\n\n"),
        },
    ];
}

#[test]
fn config_parsing_01() -> Result<(), &'static str> {
    let parsed = config::loader::parse_config(
        &String::from(CONFIG_PARSE_TO_PARSE)
    ).ok_or("Could not parse")?;
    Ok(assert_eq!(config_parse_expected_result().len(), parsed.len()))
}

#[test]
fn config_parsing_02() -> Result<(), &'static str> {
    let parsed = config::loader::parse_config(
        &String::from(CONFIG_PARSE_TO_PARSE)
    ).ok_or("Could not parse")?;
    let expected = config_parse_expected_result();
    let mut passed = true;
    for current_item in expected {
        let current_value = config::loader::search_key_in_config(
            &parsed,
            &current_item.key,
        ).ok_or("Not found")?;
        if current_item.value != current_value {
            passed = false;
            println!("Value '{}' found.", current_value);
            println!("Value '{}' failed.", current_item.key);
            break;
        }
    }
    Ok(assert_eq!(passed, true))
}
