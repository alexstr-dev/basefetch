mod utils;

use lxinfo::info;
use utils::CommandUtils;

fn main() {
    print_information();
}

///Printing system info. Commands make me want to rip my hair out
fn print_information() {
    let system_info =
        info::get_system_information().expect("Something went wrong. Please try again later!");
    let packages = CommandUtils::get_command_output("pacman -Q | wc -l");
    let username = system_info.username;
    let hostname = system_info.hostname;
    let distro = system_info.distro_name;
    let kernel = system_info.kernel;
    let shell = system_info.shell;
    let uptime = system_info.uptime_formatted;
    let used_mem = system_info.used_mem;
    let total_mem = system_info.total_mem;
    let wm = std::env::var("XDG_CURRENT_DESKTOP").expect("Failed to get WM!");
    let cpu = CommandUtils::get_command_output("cpuid -1 | rg 'brand =' | cut -d '\"' -f2");
    let gpu = CommandUtils::get_command_output(
        "lspci -v -m | rg VGA -A 7 | rg Device | sed -n '2 p' | cut -d '[' -f2 | cut -d ']' -f1",
    );
    println!("BaseFetch\n___________________");
    println!("{username}@{hostname}");
    println!("Distro: {distro}");
    println!("Kernel: {kernel}");
    println!("Packages: {packages}");
    println!("Shell: {shell}");
    println!("WM: {wm}");
    println!("CPU: {cpu}");
    println!("GPU: {gpu}");
    println!("Uptime: {uptime}");
    println!("Memory: {used_mem}/{total_mem}");
}
