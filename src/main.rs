extern crate clap;
use clap::{Arg, App, SubCommand};
use std::net::{SocketAddr, TcpStream};

// https://clap.rs/
fn ping(addrs: SocketAddr) {
    if let Ok(_stream) = TcpStream::connect(&addrs) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}

fn main() {
    let matches = App::new("tcp_scanner")
       .version("1.0")
       .about("Tcp Scanner")
       .author("Carter Tsai")
       .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .subcommand(SubCommand::with_name("scan")
            .version("1.0")
            .author("Carter Tsai <hamming1@gmail.com>")
            .arg(Arg::with_name("host:ip")
                .help("Sets the input file to use")
                .required(true)
                .index(1))
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
       .get_matches();

    if let Some(matches) = matches.subcommand_matches("scan") {
        let host_and_port = matches.value_of("host:ip").unwrap_or("");

        let addrs: SocketAddr = host_and_port
            .parse()
            .expect("Unable to parse socket address");

        if matches.is_present("debug") {
            println!("Value for host_and_port: {}", addrs);
        } else {
            println!("Printing normally...");
        }
        
        ping(addrs);
    }

    
}
