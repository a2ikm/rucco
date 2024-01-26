use std::env;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::process;
use std::str::Bytes;

struct Tokenizer<'a> {
    source: &'a str,
    source_bytes: Peekable<Enumerate<Bytes<'a>>>,
}

impl Tokenizer<'_> {
    fn next_byte(&mut self) -> Option<(usize, u8)> {
        self.source_bytes.next()
    }

    fn read_number(&mut self) -> Result<&str, String> {
        let start: usize;
        let mut len: usize = 0;

        match self.source_bytes.peek() {
            Some((i, byte)) => {
                if byte.is_ascii_digit() {
                    start = *i;
                    len += 1;
                } else {
                    return Err(format!(
                        "expected ASCII digit but got `{}` at column {}",
                        byte, i
                    ));
                }
            }
            None => return Err(format!("expected ASCII digit but no character")),
        }

        loop {
            self.source_bytes.next();

            match self.source_bytes.peek() {
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

        match self.source.get(start..(start + len)) {
            Some(number) => Ok(number),
            None => Err(format!(
                "failed to read between column {} and column {}",
                start,
                start + len
            )),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: rucco <source>");
        process::exit(1);
    }

    println!(".global main");
    println!("main:");

    let source = &args[1];
    let mut tokenizer = Tokenizer {
        source: source,
        source_bytes: source.bytes().enumerate().peekable(),
    };

    match tokenizer.read_number() {
        Ok(number) => {
            println!(" mov x0, #{}", number);
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    loop {
        match tokenizer.next_byte() {
            Some((_, b'+')) => match tokenizer.read_number() {
                Ok(number) => {
                    println!(" add x0, x0, #{}", number);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            Some((_, b'-')) => match tokenizer.read_number() {
                Ok(number) => {
                    println!(" sub x0, x0, #{}", number);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            },
            Some((i, _)) => {
                eprintln!("expected `+` or `-` at #{}", i);
                process::exit(1);
            }
            None => break,
        }
    }

    println!(" mov x8, #93");
    println!(" svc #0");
}
