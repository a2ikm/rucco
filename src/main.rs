use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rucco <exit_code>");
        process::exit(1);
    }

    let code = &args[1];

    println!(".global main");
    println!("main:");
    println!(" mov x8, #93"); // setup exit syscall
    println!(" mov x0, #{}", code); // exit code is 42
    println!(" svc #0"); // invoke syscall
}
