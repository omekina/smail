use std::env::var_os;


/**
Get absolute home folder path. Works only on Linux for now. :(
 */
pub fn get_home_folder() -> Option<String> {
    let home_dir: String = var_os("HOME")?.into_string().ok()?;
    return Some(home_dir);
}
