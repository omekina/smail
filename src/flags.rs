pub struct ArgumentFlag {
    name: String,
    value: Option<String>,
}


/**
Prase arguments flags before commands.
*/
pub fn parse_flags(arguments: &Vec<String>) -> Vec<ArgumentFlag> {
    let mut result: Vec<ArgumentFlag> = Vec::new();

    for argument in arguments {
        if argument.chars().take(2).collect::<String>() != "--" {
            break;
        }
        let flag: String = argument.chars().skip(2).collect();
        let argument_split: Vec<&str> = flag.splitn(2, "=").collect();
        if argument_split.len() == 1 {
            result.push(ArgumentFlag { name: flag.clone(), value: None });
            continue;
        }
        result.push(ArgumentFlag { name: argument_split[0].to_string(), value: Some(argument_split[1].to_string()) });
    }

    return result;
}


/**
Try to find flag by name in flags.
*/
pub fn find_flag(name: &str, flags: &Vec<ArgumentFlag>) -> Option<String> {
    for flag in flags {
        if flag.name == name { return Some( match flag.value.clone() {
            Some(value) => value,
            None => String::new(),
        }); }
    }
    return None;
}
