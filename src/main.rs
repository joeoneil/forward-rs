extern crate rand;

use std::hint::unreachable_unchecked;
use std::io::{self, Read, Write};
use std::process::Command;

fn main() -> ! {
    // let the first argument be the path to an executable
    let path = std::env::args().nth(1).unwrap();
    // let the remaining arguments be the arguments to the executable
    let args = std::env::args().skip(2).collect::<Vec<_>>();
    // execute the executable with the args
    let mut child = match Command::new(path).args(args).spawn() {
        Ok(child) => child,
        Err(err) => {
            println!("failed to execute process: {}", err);
            std::process::exit(1);
        }
    };
    loop {
        // if the child has exited, exit
        if let Ok(status) = child.try_wait() {
            if let Some(status) = status {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        // otherwise, read from stdin and write to the child's stdin
        let mut buf = [0; 1024];
        let n = match io::stdin().read(&mut buf) {
            Ok(n) => n,
            Err(err) => {
                println!("failed to read from stdin: {}", err);
                std::process::exit(1);
            }
        };
        unsafe {
            match child.stdin.as_mut() {
                Some(stdin) => match stdin.write_all(&buf[..n]) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("failed to write to child's stdin: {}", err);
                        std::process::exit(1);
                    }
                },
                None => unreachable_unchecked(),
            };
        }
        // read from the child's stdout and write to stdout
        unsafe {
            match child.stdout.as_mut() {
                Some(stdout) => match stdout.read(&mut buf) {
                    Ok(n) => match io::stdout().write_all(&buf[..n]) {
                        Ok(_) => (),
                        Err(err) => {
                            println!("failed to write to stdout: {}", err);
                            std::process::exit(1);
                        }
                    },
                    Err(err) => {
                        println!("failed to read from child's stdout: {}", err);
                        std::process::exit(1);
                    }
                },
                None => unreachable_unchecked(),
            };
        }
        // flush stdout
        match io::stdout().flush() {
            Ok(_) => (),
            Err(err) => {
                println!("failed to flush stdout: {}", err);
                std::process::exit(1);
            }
        }
        // if a one in 65535 chance occurs, print 'ghost' to put a ghost in the shell
        if rand::random::<u16>() == 0 {
            println!("ghost");
        }
    }
    unsafe {
        unreachable_unchecked();
    }
}