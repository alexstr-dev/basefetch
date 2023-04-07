use lxinfo::info;
use std::process::Command;

fn main() {
    print_information();
}

///Printing system info. Commands make me want to rip my hair out
fn print_information() {
    let system_info =
        info::get_system_information().expect("Something went wrong. Please try again later!");
    println!("BaseFetch\n___________________");
    println!("{}@{}", system_info.username, system_info.hostname);
    println!("Distribution: {}", system_info.distro_name);
    println!("Kernel: {}", system_info.kernel);
    println!("Packages: {}", get_command_output("pacman -Q | wc -l"));
    println!("Shell: {}", system_info.shell);
    println!("WM: {}", get_command_output("echo $XDG_CURRENT_DESKTOP"));
    println!(
        "CPU: {}",
        get_command_output("cpuid -1 | rg 'brand =' | cut -d '\"' -f2")
    );
    println!("GPU: {}", get_command_output("lspci -v -m | rg VGA -A 7 | rg Device | sed -n '2 p' | cut -d '[' -f2 | cut -d ']' -f1"));
    println!("Uptime: {}", system_info.uptime_formatted);
    println!("Memory: {}/{}", system_info.used_mem, system_info.total_mem);
}

///Executing a command and converting it to a String. The pop removes the extra line in the output
fn get_command_output(command: &str) -> String {
    let mut packages = String::from_utf8(
        Command::new("sh")
            .args(["-c", command])
            .output()
            .unwrap()
            .stdout,
    )
    .expect("Failed to execute command");
    packages.pop();
    packages
}
