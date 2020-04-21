use tte_lib::Profiler;

use getopts::Options;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let program_start_arg = args.iter().position(|x| !x.starts_with("-")).unwrap_or(1);
    let remaning_args = &args[program_start_arg..];

    let mut opts = Options::new();
    opts.optflag("o", "--std-out", "Print std out of process.");
    opts.optflag("e", "--extended", "Extended timing output.");
    opts.optflag("m", "--minimal",  "Only output time taken in ms");
    let matches = match opts.parse(&args[..program_start_arg]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    }; 

    let target_proc = &remaning_args[0];
    let target_args = &remaning_args[1..];
    let mut command = Command::new(target_proc);
    command.args(target_args.to_vec());

    if !matches.opt_present("o") {
       command.stdout(Stdio::null());
    }
    
    let mut profiler = Profiler::start_new();
    command.status().expect("Failed to execute process");
    profiler.stop();

    if matches.opt_present("o"){
        println!("");
        println!("");
    }

    let elapsed = profiler.elapsed();
    if matches.opt_present("m"){
        println!("{}", elapsed.total_ms());
    }else{
        if matches.opt_present("e"){
            println!("{} {}", target_proc, target_args.to_vec().join(" "));
            println!("");
            println!("Days              : {}", elapsed.days());
            println!("Hours             : {}", elapsed.hours());
            println!("Minutes           : {}", elapsed.minutes());
            println!("Seconds           : {}", elapsed.seconds());
            println!("Milliseconds      : {}", elapsed.ms());
            println!("Nanoseconds       : {}", elapsed.nanos());
            println!("");
            println!("Total Days        : {}", elapsed.total_days());
            println!("Total Hours       : {}", elapsed.total_hours());
            println!("Total Minutes     : {}", elapsed.total_minutes());
            println!("Total Seconds     : {}", elapsed.total_seconds());
            println!("Total Milliseconds: {}", elapsed.total_ms());
        }else{
            println!("Executed in {}s", elapsed.total_seconds());
        }
    }
    
}
