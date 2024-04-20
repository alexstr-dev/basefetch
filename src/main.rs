mod utils;

use lxinfo::info;
use utils::CommandUtils;

fn main() {
    print_information();
}

///Printing system info. Commands make me want to rip my hair out
fn print_information() {
    let to_print = replace_keys_and_return();
    println!("{to_print}");
}

///Replacing keys in the config file and returning to print
fn replace_keys_and_return() -> String {
    let system_info = info::get_system_information().expect("Something went wrong. Please try again later!");
    let output = CommandUtils::get_command_output("cat ~/.config/basefetch/config");
    let username = system_info.username;
    let hostname = system_info.hostname;
    let distro = system_info.distro_name;
    let kernel = system_info.kernel;
    let shell = system_info.shell;
    let uptime = system_info.uptime_formatted;
    let used_mem = system_info.used_mem;
    let total_mem = system_info.total_mem;
    let available_mem = system_info.available_mem;
    let wm = std::env::var("XDG_CURRENT_DESKTOP").expect("Failed to get WM!");
    let cpu = CommandUtils::get_command_output("cpuid -1 | rg 'brand =' | cut -d '\"' -f2");
    let gpu = CommandUtils::get_command_output("lspci -v -m | rg VGA -A 7 | rg Device | sed -n '1 p' | cut -d '[' -f2 | cut -d ']' -f1");
    let fuck_off = output.replace("{username}", &username)
    .replace("{distro}", &distro)
    .replace("{hostname}", &hostname)
    .replace("{kernel}", &kernel)
    .replace("{shell}", &shell)
    .replace("{uptime}", &uptime)
    .replace("{wm}", &wm)
    .replace("{used_mem}", &used_mem)
    .replace("{total_mem}", &total_mem)
    .replace("{available_mem}", &available_mem)
    .replace("{cpu}", &cpu)
    .replace("{gpu}", &gpu);
    return fuck_off;
}
