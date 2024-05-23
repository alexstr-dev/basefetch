mod utils;

use lxinfo::info;
use utils::CommandUtils;
use std::fs::{read_to_string, self};

fn main() {
    print_information();
}

///Replacing keys in the config file and returning to print
fn print_information() {
    let system_info = info::get_system_information().expect("Something went wrong. Please try again later!");
    let path = format!("{}/.config/basefetch/config", std::env::var("HOME").expect("Cant find config or path"));
    let mut config = read_to_string(path).expect("An error has occured, please check your config");
    config = config.replace("{username}", &system_info.username)
    .replace("{distro}", &system_info.distro_name)
    .replace("{hostname}", &system_info.hostname)
    .replace("{kernel}", &system_info.kernel)
    .replace("{shell}", &system_info.shell)
    .replace("{uptime}", &system_info.uptime_formatted)
    .replace("{wm}", &std::env::var("XDG_CURRENT_DESKTOP").expect("Failed to get WM!"))
    .replace("{used_mem}", &system_info.used_mem)
    .replace("{total_mem}", &system_info.total_mem)
    .replace("{available_mem}", &system_info.available_mem)
    .replace("{cpu}", &CommandUtils::get_command_output("cpuid -1 | rg \'brand =\' | cut -d \'\"\' -f2"))
    .replace("{gpu}", &CommandUtils::get_command_output("lspci | grep \"VGA\" | cut -d'[' -f2 | cut -d']' -f1"))
    .replace("{packages}", &fs::read_dir("/var/lib/pacman/local").expect("Error: Directory not found!").count().to_string())
    .replace("{flatpak}", &CommandUtils::get_command_output("flatpak list | wc -l"));
    println!("{config}");
}
