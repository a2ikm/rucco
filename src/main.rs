use std::env;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::process;
use std::str::Bytes;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rucco <exit_code>");
        process::exit(1);
    }

    let source = &args[1];
    let mut source_bytes = source.bytes().enumerate().peekable();

    match read_number(source, &mut source_bytes) {
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

fn read_number<'a>(
    source: &'a str,
    source_bytes: &mut Peekable<Enumerate<Bytes<'_>>>,
) -> Option<&'a str> {
    let start: usize;
    let mut len: usize = 0;

    match source_bytes.peek() {
        Some((i, byte)) => {
            if byte.is_ascii_digit() {
                start = *i;
                len += 1;
            } else {
                return None;
            }
        }
        None => return None,
    }

    loop {
        source_bytes.next();

        match source_bytes.peek() {
            Some((_, byte)) => {
                if byte.is_ascii_digit() {
                    len += 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    source.get(start..(start + len))
}
