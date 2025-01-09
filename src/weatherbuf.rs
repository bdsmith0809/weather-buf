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

pub mod weather {

    use std::net::{UdpSocket, SocketAddr};
    use std::str::FromStr;
    use std::thread;
    use std::time::Duration;
    use messages::WeatherData;
    use prost::Message;
    use rand::Rng;
    use std::path::Path;
    use std::fs;
    use std::io::{self, Cursor, Read, Write};

    pub mod messages {
        include!(concat!(env!("OUT_DIR"), "/messages.rs"));
    }

    /// Print the contents of a message to stdout
    pub fn print_msg(msg_rcvd: &WeatherData) {
        println!("Weather message:");
        println!("  Station: {}", msg_rcvd.station_name());
        println!("  Temp: {}", msg_rcvd.temperature());
        println!("  Humidity: {}", msg_rcvd.relative_humidity());
        println!("  Wind: {}", msg_rcvd.wind_speed());            
        println!("  Direction: {}", msg_rcvd.wind_direction());
    }

    /// Write a length-delimited message to binary file, appending to the end of the file
    pub fn write_msg_to_file(outfile_path: &Path, msg_out: messages::WeatherData) -> io::Result<()> {
        println!("Writing Weatherdata with encoded size: {}", msg_out.encoded_len());
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(outfile_path)?;
        let buf = serialize_weather_msg(msg_out);

        file.write_all(&buf)?;
        
        Ok(())
    }
  
    /// Read length-delimited WeatherData messages from a binary file and return as a vector
    pub fn read_msgs_from_file(outfilename: &Path) -> Result< Vec<messages::WeatherData>, Box<dyn std::error::Error >> {
        
        let mut file = fs::File::open(outfilename)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut cursor = Cursor::new(buf);
        let mut msg_vec: Vec<WeatherData> = Vec::new();
        while let Ok(msg) = WeatherData::decode_length_delimited(&mut cursor) {
            msg_vec.push(msg);
        }

        println!("Read {} messages from the file.", msg_vec.len());
        Ok(msg_vec)

    }

    /// Encode a Weather message, including a message length delimiter
    fn serialize_weather_msg(weather_msg: messages::WeatherData) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(weather_msg.encoded_len());
        // Unwrap is safe, since we have reserved sufficient capacity in the vector.
        weather_msg.encode_length_delimited(&mut buf).unwrap();
        buf
    }

        
    /// Decode the length-delimited message
    fn deserialize_weather_msg(buf: &[u8]) -> Result<messages::WeatherData, prost::DecodeError> {
        messages::WeatherData::decode_length_delimited(&mut Cursor::new(buf))
    }

    /// Generate a Weather message using some randomized data
    pub fn generate_weather_msg() -> messages::WeatherData {
     
        // Generate a random f64 between 0.0 (inclusive) and 1.0 (exclusive)
        let mut rng = rand::thread_rng();
        let mut rndm_val: f64 = rng.gen();
        let random_temp: f64 = 70.0 + 10.0*rndm_val;  // random temp between 70.0-80.0 deg F
        let rndm_f32: f32 = rng.gen();
        let random_humid: f32 = 0.3 + 0.1*rndm_f32;  // random relative humidity between 0.3 - 0.4, 32-bit float
        rndm_val = rng.gen();
        let random_wind: f64 = 0.0 + 10.0*rndm_val;  // random wind speed between 0.0 and 10.0 mph

        return messages::WeatherData {
            station_name: Some("A10H7B".to_string()), 
            temperature: Some(random_temp), 
            relative_humidity: Some(random_humid), 
            wind_speed: Some(random_wind), 
            wind_direction: Some("N".to_string())
        }
    }

    /// Run a client that listens for incoming Weather messages.
    /// 
    /// Arguments:
    /// ip_addr     String. Use format: [IPv4]:[port]
    /// 
    /// Specify the same IP address to which the 'reporter' client
    /// will send its messages.
    pub fn run_listener(ip_addr: &str) -> std::io::Result<()> {
        let socket = UdpSocket::bind(ip_addr)?;
        println!("Listening for weather messages on {}", ip_addr);

        let mut buf = [0u8; 1024];

        loop {
            let (_len, _src) = socket.recv_from(&mut buf)?;

            //println!("Received from {}: {:?}", src, &buf[..len]);

            let msg_rcvd = deserialize_weather_msg(&buf)?;
            print_msg(&msg_rcvd);            
        }
    }    
    
    /// Run a client that generates Weather messages and reports them
    /// over UDP to the specified listener located at the provided
    /// IP Address.
    /// 
    /// Arguments:
    /// 
    /// listener_ip_addr:   String. Use format: [IPv4]:[port]
    /// 
    /// This should be the same address and port as provided to run_listener.
    pub fn run_reporter(listener_ip_addr: &str) -> std::io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?; // Bind to any available port
    
        let _server = SocketAddr::from_str(listener_ip_addr);

        const MESSAGE_INTERVAL_MS: u64 = 1000;
    
        loop {
            
            let weather_msg = generate_weather_msg();
            let buf = serialize_weather_msg(weather_msg);
            //println!("Sending message with bytes: {:?}", buf);
            println!("Sending weather message");
    
            socket.send_to(&buf, listener_ip_addr)?;

            thread::sleep(Duration::from_millis(MESSAGE_INTERVAL_MS)); // Sleep for 5 seconds
    
        }
    }

    #[cfg(test)]
    mod tests {
        
        use std::path::Path;

        use super::{read_msgs_from_file, generate_weather_msg, serialize_weather_msg, deserialize_weather_msg,
            write_msg_to_file};

        /// Test serialization and deserialization of Weather protobuf message
        #[test]
        fn test_serialize() {

            let mut weather_msg = generate_weather_msg();
            // Set some values
            weather_msg.station_name = Some("TEST_STATION".to_string());
            weather_msg.temperature = Some(98.6);

            let buf = serialize_weather_msg(weather_msg);
            let new_msg = match deserialize_weather_msg(&buf){
                Ok(new_msg) => new_msg,
                Err(e) => panic!("Error decoding message: {}", e)
            };
            assert_eq!("TEST_STATION", new_msg.station_name());
            assert_eq!(98.6, new_msg.temperature());

    }

        /// Test writing messages to a file, and reading the messages back into a vector
        #[test]
        fn test_file_io() {

            let infilename = Path::new("./_tmp_messages_written.bin");
            
            if infilename.exists() {
                match std::fs::remove_file(infilename) {
                    Ok(_) => (),
                    Err(e) => panic!("Unable to delete existing temp file: {}", e)
                };
            } 
            
            let num_msg = 10;
            for _ii in 0..num_msg {
                let this_msg = generate_weather_msg();
                match write_msg_to_file(infilename, this_msg) {
                    Ok(_) => (),
                    Err(e) => panic!("Unable to write to file: {}", e)
                };
            }

            let msg_vec = match read_msgs_from_file(infilename){
                Ok(msg_vec) => msg_vec,
                Err(e) => panic!("Help me: {}", e)
            };

            assert!(msg_vec.len() == (num_msg as usize));  

            if infilename.exists() {
                match std::fs::remove_file(infilename) {
                    Ok(_) => (),
                    Err(e) => panic!("Unable to delete existing temp file: {}", e)
                };
            }         

        }
    }

}
