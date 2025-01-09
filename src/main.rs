/*
weather-buf  A simple program showing protobuf implementation in Rust.

    Copyright (C) 2024  Brian Smith

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod weatherbuf;

use clap::{arg, Command};

/// Spawn a client that sends or receives Weather protobuf messages over UDP.
/// 
/// Example:
/// 1) In a terminal, spawn a listener at the specified IP address and port:
///     weather-buf --appmode listener --addr 127.0.0.1:5678
/// 
/// 2) In a different terminal, spawn a reporter, and specify the address of
/// the listener to which Weather messages are sent.
///     weather-buf --appmode reporter --addr 127.0.0.1:5678
/// 
/// The reporter will continually send messages at a fixed interval.
/// In the listener window, the Weather message data will be printed
/// upon receipt.
fn main() {
    let matches = Command::new("weatherbuf")
        .version("0.1")
        .about("A very amateur example of Rust protobuf messaging using a UDP sender/receiver.")
        .arg(arg!(--appmode <VALUE>)
            .required(true))
        .arg(arg!(--address <VALUE>)
            .required(true))
        .get_matches();
    
    let appmode = matches.get_one::<String>("appmode").expect("required");
    let address = matches.get_one::<String>("address").expect("required");
    if appmode == "reporter" {
        let _result = weatherbuf::weather::run_reporter(address.as_str());
    } else if appmode == "listener"  {
        let _result = weatherbuf::weather::run_listener(address.as_str());
    }else {
        println!("Valid appmode option is 'reporter' or 'listener'.");
    }
    
}