use console::Term;
use owo_colors::OwoColorize;
use std::io::{Read, Write};
use std::process::{exit, Command};

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    let cmd = if args.len() > 1 {
        args[1].to_string()
    } else {
        eprintln!(
            "{} {}",
            get_prefix().red(),
            "no command specified!".bright_magenta()
        );
        exit(1);
    };

    args.drain(0..2);
    let args = args;

    let hostname = match Command::new("hostname").output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(hostname) => hostname.trim().to_string(),
            Err(_) => "localhost".to_string(),
        },
        Err(_) => "localhost".to_string(),
    };

    let mut command = Command::new(cmd.clone());
    command.args(args.clone());

    // is there a cleaner way to do this?
    let command_string = format!("{} {}", cmd, args.join(" "));

    println!(
        "{} {} {}.",
        get_prefix().red(),
        "your current hostname is:".yellow(),
        hostname.bright_green().bold()
    );
    print!(
        " {} {} {} {}{} {} ",
        "do you".yellow(),
        "*REALLY*".red().italic().bold(),
        "want to run".yellow(),
        command_string.trim().red(),
        "?".yellow(),
        "(y/n)".yellow()
    );

    // flush stdout to display the prompt
    let _ = std::io::stdout().flush();

    let c = read_char();
    if c == 'y' || c == 'Y' {
        // children bad, only use them if its *REALLY* necessary (only unix supports exec)
        #[cfg(not(unix))] {
            let mut child = command.spawn().expect("failed to run command");
            let status = child.wait().expect("failed to wait on child");
            exit(status.code().unwrap());
        }
        #[cfg(unix)] {
            use std::os::unix::process::CommandExt;
            let error = command.exec();
            println!("{} {}", get_prefix().red(), error);
        }
    } else {
        println!(
            "{} {}",
            get_prefix().red(),
            "oki, won't execute!".bright_magenta()
        );
    }
}

fn get_prefix() -> String {
    format!("[{}]", env!("CARGO_CRATE_NAME"))
}

fn read_char() -> char {
    match Term::stdout().read_char() {
        Ok(c) => {
            println!();
            c
        }
        Err(_) => std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as char)
            .unwrap(),
    }
}
