/*
 *
 *  MIT License
 *
 *  Copyright (c) [2021] [Thomas Thiel]
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a copy
 *  of this software and associated documentation files (the "Software"), to deal
 *  in the Software without restriction, including without limitation the rights
 *  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  copies of the Software, and to permit persons to whom the Software is
 *  furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in all
 *  copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  SOFTWARE.
 */


use clap::{Arg, App};
use regex::RegexSet;
use regex::Regex;

fn main() {

    // parse cmd line options
    let matches = App::new("macconv")
        .version("1.0.0")
        .author("Thomas Thiel <kv@wolfchild.de>")
        .about("normalizes and converts mac addresses.")
        .arg(Arg::with_name("mac")
                 .help("A MAC Address to work on")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("dashed")
                 .short("d")
                 .long("dashed")
                 .takes_value(false)
                 .help("output mac in dashed notation"))
        .arg(Arg::with_name("colon")
                 .short("c")
                 .long("colon")
                 .takes_value(false)
                 .help("output mac in colon notation"))
        .arg(Arg::with_name("cisco")
                 .short("w")
                 .long("cisco")
                 .takes_value(false)
                 .help("output mac in Cisco notation"))
        .arg(Arg::with_name("caps")
                 .short("C")
                 .long("caps")
                 .takes_value(false)
                 .help("output mac using capital letters"))
        .get_matches();

    // prepare input variable
    let input = matches.value_of("mac").unwrap();

    // prepare regular expressions for input checking
    let mac_verifier = RegexSet::new(&[
        r#"^[0-9a-fA-F]{12}$"#,
        r#"^([0-9a-fA-F]{2}[:\-.]){5}[0-9a-fA-F]{2}$"#,
        r#"^([0-9a-fA-F]{4}[.]){2}[0-9a-fA-F]{4}$"#
    ]).unwrap();

    // check if input is valid
    if mac_verifier.is_match(input) {
        // remove all delimiters
        let re = Regex::new(r"[-:.]").unwrap();
        let normalizedmac = re.replace_all(input, "");

        // prepare mac byte storage
        let mut parsed_mac_address: [u8; 6] = [0,0,0,0,0,0];

        // parse input into byte storage
        parsed_mac_address[0]=u8::from_str_radix(&normalizedmac[0..2], 16).unwrap();
        parsed_mac_address[1]=u8::from_str_radix(&normalizedmac[2..4], 16).unwrap();
        parsed_mac_address[2]=u8::from_str_radix(&normalizedmac[4..6], 16).unwrap();
        parsed_mac_address[3]=u8::from_str_radix(&normalizedmac[6..8], 16).unwrap();
        parsed_mac_address[4]=u8::from_str_radix(&normalizedmac[8..10], 16).unwrap();
        parsed_mac_address[5]=u8::from_str_radix(&normalizedmac[10..], 16).unwrap();

        // we need to know if the user is lazy
        let mut fallback = false;

        // so if the user was lazy with parameters, lets detect that and mark it 
        // by setting fallback=true.
        if (!matches.is_present("cisco"))  &
           (!matches.is_present("dashed")) &
           (!matches.is_present("colon"))
        {
            fallback=true;
        };

        // print output regarding to users wishes
        if matches.is_present("caps") {
            if matches.is_present("cisco") {
                println!("{:02X}{:02X}.{:02X}{:02X}.{:02X}{:02X}",
                         parsed_mac_address[0],
                         parsed_mac_address[1],
                         parsed_mac_address[2],
                         parsed_mac_address[3],
                         parsed_mac_address[4],
                         parsed_mac_address[5]);
            };
            // if the user did specify nothing else but the case, we default here...
            if matches.is_present("colon") | fallback {
                println!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                         parsed_mac_address[0],
                         parsed_mac_address[1],
                         parsed_mac_address[2],
                         parsed_mac_address[3],
                         parsed_mac_address[4],
                         parsed_mac_address[5]);
            };
            if matches.is_present("dashed") {
                println!("{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
                         parsed_mac_address[0],
                         parsed_mac_address[1],
                         parsed_mac_address[2],
                         parsed_mac_address[3],
                         parsed_mac_address[4],
                         parsed_mac_address[5]);
            };
        } else {
            if matches.is_present("cisco") {
                println!("{:02x}{:02x}.{:02x}{:02x}.{:02x}{:02x}",
                         parsed_mac_address[0],
                         parsed_mac_address[1],
                         parsed_mac_address[2],
                         parsed_mac_address[3],
                         parsed_mac_address[4],
                         parsed_mac_address[5]);
            };
            // here we use the fallback again! this will be our default...
            if matches.is_present("colon") | fallback {
                println!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                         parsed_mac_address[0],
                         parsed_mac_address[1],
                         parsed_mac_address[2],
                         parsed_mac_address[3],
                         parsed_mac_address[4],
                         parsed_mac_address[5]);
            };
            if matches.is_present("dashed") {
                println!("{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
                         parsed_mac_address[0],
                         parsed_mac_address[1],
                         parsed_mac_address[2],
                         parsed_mac_address[3],
                         parsed_mac_address[4],
                         parsed_mac_address[5]);
            };
        };

    // input was not valid!
    } else {
        println!("Error: {:?} is not a valid MAC Address!", input);
        std::process::exit(201);
    };

}


