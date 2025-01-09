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

#[cfg(test)]
mod test {

    use std::path;
    use super::weatherbuf;

    /// Test writing and reading WeatherData messages from a file. This test
    /// is basically the same as a similar test in weatherbuf.rs.
    #[test]
    fn test_file_write() {

        println!("Hello protobuf!");
        let infilename = path::Path::new("./_tmp_messages_written.bin");
                
        let num_msg = 10;
        for _ii in 0..num_msg {
            let this_msg = weatherbuf::weather::generate_weather_msg();
            match weatherbuf::weather::write_msg_to_file(infilename, this_msg) {
                Ok(_) => (),
                Err(e) => panic!("Unable to write to file: {}", e)
            };
        }

        let msg_vec = match weatherbuf::weather::read_msgs_from_file(infilename){
            Ok(msg_vec) => msg_vec,
            Err(e) => panic!("Help me: {}", e)
        };

        for the_msg in msg_vec.iter() {
            weatherbuf::weather::print_msg(the_msg);
        }

        assert!(msg_vec.len() == num_msg as usize);

        if infilename.exists() {
            match std::fs::remove_file(infilename) {
                Ok(_) => (),
                Err(e) => panic!("Unable to delete existing temp file: {}", e)
            };
        }

    }

}