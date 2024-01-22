use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rucco <exit_code>");
        process::exit(1);
    }

    let source = &args[1];
    match read_number(source) {
        Some(code) => {
            println!(".global main");
            println!("main:");
            println!(" mov x8, #93"); // setup exit syscall
            println!(" mov x0, #{}", code); // exit code is 42
            println!(" svc #0"); // invoke syscall
        }
        None => {
            eprintln!("Usage: rucco <exit_code>");
            process::exit(1);
        }
    }
}

fn read_number(source: &str) -> Option<&str> {
    let mut bytes = source.bytes();
    let mut len: usize = 0;

    loop {
        match bytes.next() {
            Some(byte) => {
                if byte.is_ascii_digit() {
                    len += 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    source.get(0..len)
}
