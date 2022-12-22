use clap::{value_parser, App, Arg, ArgAction, ArgMatches, Command};
use colored::*;
use std::{
    error::Error,
    time::{Duration, Instant},
};

mod tcp;
pub use tcp::write;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = new_cmd();
    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("request", request_matches)) => {
            request_cmd(request_matches)?;
        }

        _ => unreachable!(),
    }

    Ok(())
}

fn request_cmd(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let addrs: Vec<&str> = matches.values_of("addr").unwrap().collect();
    let data = matches.value_of("data").unwrap();
    let multiple: usize = match matches.get_one::<usize>("multiple-data") {
        Some(x) => *x,
        None => 1,
    };
    let repeat: usize = match matches.get_one::<usize>("repeat") {
        Some(x) => *x,
        None => 1,
    };

    let data_multiplied = data.repeat(multiple);
    let sent = data_multiplied.trim();

    let request_count = addrs.len() * repeat;
    let mut total_duration = Duration::new(0, 0);
    let (mut passes, mut fails, mut i) = (0, 0, 0);
    for addr in addrs {
        for _ in 0..repeat {
            i = i + 1;
            println!(
                "\n{} {} - {i}/{request_count}",
                "#".blue(),
                "REQUEST".blue(),
            );
            println!("{}:\t\t{}", "Server".blue(), addr);
            println!("{}:\t\t{}", "Sent".blue(), sent);

            let start = Instant::now();
            let response = write(addr, &sent);
            let duration = start.elapsed();
            match response {
                Ok(data) => {
                    println!("{}:\t{}", "Received".blue(), data);
                    println!("{}:\t{} ms", "R+W Duration".blue(), duration.as_millis());
                    total_duration = total_duration + duration;

                    if sent == data {
                        passes = passes + 1;
                        println!(
                            "{} - The data sent and received are the same!",
                            "OK".green()
                        );
                    } else if sent.to_uppercase() == data {
                        passes = passes + 1;
                        println!(
                            "{} - The data sent and received are the same! (Uppercase)",
                            "OK".green()
                        );
                    } else {
                        fails = fails + 1;
                        println!("{} - NOT SAME!", "FAIL".red());
                    }
                }
                Err(err) => {
                    fails = fails + 1;
                    println!("{}:\tNone", "Received".blue());
                    println!("{} - {}", "FAIL".red(), err);
                    continue;
                }
            };
        }
    }

    println!(
        "\n{}\n{passes} {}, {fails} {}",
        "# RESULT".blue(),
        "passes".green(),
        "fails".red()
    );
    println!(
        "{}: {} ms\n",
        "Average R+W Duration".blue(),
        total_duration.as_millis() / passes
    );
    Ok(())
}

fn new_cmd() -> App<'static> {
    return Command::new("echo-test")
        .about("Echo Server Testing Program")
        .version("0.1.0")
        .subcommand_required(true)
        .author("Bora Tanrikulu <me@bora.sh>")
        .subcommand(
            Command::new("request")
                .alias("r")
                .alias("req")
                .about("Send TCP package to the server and take the response")
                .arg(
                    Arg::new("addr")
                        .short('a')
                        .long("addr")
                        .help("Address of the TCP server")
                        .action(ArgAction::Set)
                        .required(true)
                        .multiple_values(true)
                        .value_delimiter(','),
                )
                .arg(
                    Arg::new("data")
                        .short('d')
                        .long("data")
                        .help("Data to sent")
                        .action(ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("multiple-data")
                        .short('m')
                        .long("multiple-data")
                        .help("How many times to multiple the data")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    Arg::new("repeat")
                        .short('r')
                        .long("repeat")
                        .help("How many times to send the request")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(usize)),
                ),
        );
}
