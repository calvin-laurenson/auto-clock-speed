use daemon::{daemon_init, Checker};
use display::{
    print_available_governors, print_cpu_governors, print_cpu_speeds, print_cpus, print_freq,
    print_turbo,
};
use error::{Error, GovGetError, GovSetError, SpeedGetError, SpeedSetError};
use std::process::exit;
use structopt::StructOpt;
use system::{
    check_available_governors, check_cpu_freq, check_cpu_name, check_turbo_enabled,
    list_cpu_governors, list_cpu_speeds, list_cpus,
};
use power::{read_lid_state, read_battery_charge, read_power_source};

pub mod cpu;
pub mod daemon;
pub mod display;
pub mod error;
pub mod system;
pub mod power;

#[derive(StructOpt)]
#[structopt(
    name = "autoclockspeed",
    about = "Automatic CPU frequency scaler and power saver"
)]
enum Command {
    /// The overall frequency of your cpu
    #[structopt(name = "get-freq")]
    GetFreq {
        #[structopt(short, long)]
        raw: bool,
    },

    #[structopt(name = "power")]
    Power {
    },

    /// Get whether turbo is enabled or not
    #[structopt(name = "get-turbo")]
    GetTurbo {
        #[structopt(short, long)]
        raw: bool,
    },

    /// Get the available governor
    #[structopt(name = "get-available-governors")]
    GetAvailableGovernors {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The names of the core
    #[structopt(name = "get-cpus")]
    GetCPUS,

    /// The speed of the individual cores
    #[structopt(name = "get-cpu-speeds")]
    GetSpeeds {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The governors of the individual cores
    #[structopt(name = "get-cpu-governors")]
    GetGovernors {
        #[structopt(short, long)]
        raw: bool,
    },

    /// Run the daemon, this checks and edit your cpu's speed
    #[structopt(name = "run")]
    Run {
        /// Show the information the monitor sub-command outputs
        #[structopt(short, long)]
        verbose: bool,

        /// Milliseconds between update
        #[structopt(short, long, default_value = "1000")]
        delay: u64,
    },

    /// Monitor each cpu, it's min, max, and current speed, along with the governor
    #[structopt(name = "monitor")]
    Monitor {
        /// Milliseconds between update
        #[structopt(short, long, default_value = "1000")]
        delay: u64,
    },
}

fn main() {
    match Command::from_args() {
        Command::GetFreq { raw } => match check_cpu_freq() {
            Ok(f) => print_freq(f, raw),
            Err(_) => eprintln!("Faild to get cpu frequency"),
        },
        Command::Power {} => match read_lid_state() {
            Ok(f) => {
                match read_battery_charge() {
                    Ok(c) => {
                        match read_power_source() {
                            Ok(p) => {
                                println!("Lid: {} Bat: {} Plugged in: {}", f, c, p)
                            },
                            Err(_) => eprintln!("Faild to get read power source"),
                        }
                    },
                    Err(_) => eprintln!("Faild to get read battery charger"),
                }

            },
            Err(_) => eprintln!("Faild to get read lid state"),
        },
        Command::GetTurbo { raw } => match check_turbo_enabled() {
            Ok(turbo_enabled) => print_turbo(turbo_enabled, raw),
            Err(_) => println!("Failed to get turbo status"),
        },
        Command::GetAvailableGovernors { raw } => match check_available_governors() {
            Ok(available_governors) => print_available_governors(available_governors, raw),
            Err(_) => println!("Failed to get available governors"),
        },
        Command::GetCPUS {} => match list_cpus() {
            Ok(cpus) => match check_cpu_name() {
                Ok(name) => print_cpus(cpus, name),
                Err(_) => println!("Failed get list of cpus"),
            },
            Err(_) => println!("Failed get list of cpus"),
        },
        Command::GetSpeeds { raw } => match list_cpu_speeds() {
            Ok(cpu_speeds) => print_cpu_speeds(cpu_speeds, raw),
            Err(_) => println!("Failed to get list of cpu speeds"),
        },
        Command::GetGovernors { raw } => match list_cpu_governors() {
            Ok(cpu_governors) => print_cpu_governors(cpu_governors, raw),
            Err(_) => println!("Failed to get list of cpu governors"),
        },
        Command::Run { verbose, delay } => match daemon_init(verbose, delay, true) {
            Ok(mut d) => {
                d.run();
            }
            Err(_) => eprint!("Could not run daemon in edit mode"),
        },
        Command::Monitor { delay } => match daemon_init(true, delay, false) {
            Ok(mut d) => {
                d.run();
            }
            Err(_) => eprint!("Could not run daemon in monitor mode"),
        },
    }
}
