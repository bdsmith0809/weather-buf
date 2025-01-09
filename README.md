# weather-buf

This is an example program using some basic Rust concepts:
- clap command line argument parser
- UDP communication between a streaming broadcaster and a listener
- protobuf messages compiled into Rust using prost and prost-build
- a sleep function to control the interval at which messages are generated
- File I/O to record received messages to a json and a binary file

The weather.proto file demonstrates a very trivial example message containing
basic weather data. It has no significance to any real-world weather reporting
system.

Note there are some functions in weatherbuf.rs that are covered by unit tests,
but not necessarily used in the main.rs. These unused functions may show as warnings
when building the crate.

Note this is an entirely novice effort, as I try to learn Rust!

# Usage:

    weather-buff --appmode <VALUE> --address <VALUE>

    where:

    appmode: 'reporter' or 'listener'.

        reporter: generates a continuous stream of WeatherData messages at a constant rate.
        listener: listens for WeatherData messages and prints them to stdout

    address: [IPv4 address]:[port]

        An IPv4 address and port. In reporter appmode, this address/port is the destination
        for reported WeatherData messages. In listener appmode, this address/port will be
        the destination address, monitored for arriving WeatherData messages.

This software is for educational purposes only. No guarantee provided for its correctness
or utility.

# Example use case:

1) In a terminal, start the listener mode app at the destination address/port:

    weather-buf listener 127.0.0.1:3456

2) In a different terminal, start the reporter mode app, sending messages to the destination:

    weather-buf reporter 127.0.0.1:3456

3) The reporter app should begin sending messages at a prescribed rate.

4) The listener app should begin receiving messages and printing data to stdout.