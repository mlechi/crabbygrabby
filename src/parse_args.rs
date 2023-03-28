#![allow(unused)]
use crate::{ScanRequest, ScanType};
use std::env;

pub fn parse()-> ScanRequest{
    //Collect args. In future, consider passing args: Vec<String> in as a parameter.
    let args:Vec<String> = env::args().collect();
    //Below are the indexes of the args for ports and addresses. They will always be the args preceded by -p and -t.
    let mut arg_index_port:usize = 0;
    let mut arg_index_address:usize = 0;
    //Below is the index of the scan type. Always preceded by -st.
    let mut arg_index_scan_type:usize = 0;
    //Initialize the ScanType
    let mut s_t: ScanType = ScanType::Normal;
    //Find the indexes.
    //println!("Arguments given: {:?}",args);
    if args.len() > 1 {
        for i in 0..args.len(){
            match args[i].as_str(){
                //port
                "-p" => arg_index_port = i+1,
                //ipv4 target
                "-t" => arg_index_address = i+1,
                //Scan type
                "-st" => {match args[i+1].as_str(){
                    "c" => s_t = ScanType::Normal,
                    "s" => s_t = ScanType::Syn,
                    _ => s_t = ScanType::Normal,
                }},
                _  => (),
            }
        }
        //Use ports_parse
        let prt:Vec<i32> = ports_parse(&args[arg_index_port]);
        //Use address_parse to get list of addresses to scan.
        //As of now, address_parse does nothing.
        let mut addrs:Vec<String> = address_parse(&args[arg_index_address]);
        //Return the ScanRequest.
        ScanRequest { ports: (prt), target_addresses: (addrs), scan_type: (s_t), }
    } else {
        //Return the ScanRequest.
        let prt:Vec<i32> = vec![0];
        let addrs:Vec<String> = vec!["0.0.0.0".to_string()];
        println!("A simple port scanner for the purpose of study and practice.");
        println!("");
        println!("");
        println!("Input works as follows:");
        println!("");
        println!("Ip address is entered after the -t flag.");
        println!("    Only IPv4 addreses work right now.");
        println!("    You can enter one address, or multiple seperated by commas with no spaces.");
        println!("    Ranges, subnets, and multiple addresses seperated by commas with spaces will come later.");
        println!("");
        println!("Ports are entered after the -p flag.");
        println!("    Ports can be entered singularly, with commas, or as ranges. No spaces.");
        println!("");
        println!("Scan type is entered after -st flag.");
        println!("    This can and should be ommitted, because it does nothing as of now.");
        println!("    Right now, only a normal TCP connect scan is fully functional.");
        println!("    To do a TCP connect scan, type \"c\" after -st flag.");
        s_t = ScanType::NoScan;
        ScanRequest { ports: (prt), target_addresses: (addrs), scan_type: (s_t), }
    }
}

//IPv4 addresses only
fn address_parse(input: &String)->Vec<String>{
    //Placeholder that works on only one string, and does no input validation.
    /*let mut num_periods:u16 = 0;
    for i in input.chars() {
        if i == '.' {num_periods+=1;}
        else if !i.is_numeric(){ panic!("Only numbers and periods accepted in IP address right now."); }
    }
    if num_periods != 3 {panic!("Invalid IP address.");}
    vec![input.to_string()]*/
    let mut output:Vec<String> = Vec::new();
    let mut octet_number: u16 = 0;
    let mut octet_buffer:String = String::new();
    let mut add_buffer:String = String::new();
    for i in input.chars() {
        if i.is_numeric() {octet_buffer.push(i);}
        else if i == '.' {
            octet_buffer.push(i);
            add_buffer.push_str(octet_buffer.as_str());
            octet_buffer.clear();
            octet_number += 1;
            //num_periods += 1;
        }
        else if octet_number == 3 && (i == '-' || i == '/') {octet_buffer.push(i);}
        else if octet_number == 3 && i == ',' {
            add_buffer.push_str(octet_buffer.as_str());
            output.push(add_buffer.clone());
            add_buffer.clear();
            octet_buffer.clear();
            octet_number = 0;
        }
        else {panic!("Invalid IP address. (Invalid input)");}
    }
    add_buffer.push_str(&octet_buffer.as_str());
    println!("{}", octet_number);
    //println!("Before range parsing: {:?}",output);
    //let add_split = add_buffer.split("-");
    //let start = add_split.next().unwrap().parse::<u16>().unwrap();
    //let end = add_split.next().unwrap().parse::<u16>().unwrap();
    output.push(add_buffer.clone());
    println!("Before range parsing: {:?}",output);
    output
}

//There absolutely must be a more efficient way to do this.
fn ports_parse(list:&String)->Vec<i32>{
    //Called on arg preceded by -p.
    let mut ports:Vec<i32> = Vec::new();
    let mut loop_buffer:Vec<char> = Vec::new();
    let mut port_buffer:Vec<String> = Vec::new();
    //This will result in port_buffer being a Vec<String>, with each String being either a port or a range of ports.
    //Check for improper input, convert to ints and push to ports vector, and process ranges and push ranges to ports later.
    for i in list.chars() {
        //if i is a number, push to end of loop_buffer.
        if i.is_numeric() {
            loop_buffer.push(i);
        } else if i==','{
          //If i is a comma, push loop_buffer to port_buffer, clear loop_buffer.
          port_buffer.push(loop_buffer.iter().collect());
          loop_buffer.clear();
        } else if i=='-'{
          //If i is a dash, treat it like a number. This way, ranges are included in port_buffer to be parsed later.
          loop_buffer.push(i);
        } else if !i.is_numeric(){
          println!("Unparseable input detected. Probably gonna panic soon.");
        }
    }
    port_buffer.push(loop_buffer.iter().collect());
    //Now, loop through port_buffer, parse numeric strings to i32s, and parse ranges.
    for s in port_buffer{
        match s.parse::<i32>() {
        //if s is a number, push to ports. Use s.parse() to check and to convert to i32 at the same time.
            Ok(res) => {
               if 0<res && res<65536 {
                    ports.push(res);
                    } else {println!("Error: Ports must be between 0 and 65,535. {}", res);}
            },
        //if s.parse fails, then s.contains("-") to check if it is a range.
        Err(e) => {
            if s.contains("-") {
                //Parse range
                let mut port_split = s.split("-");
                let start_range = port_split.next().unwrap().parse::<i32>().unwrap();
                let end_range = match port_split.next().unwrap().parse::<i32>(){
                    Ok(res) => res,
                    Err(err) => {println!("You have a hanging dash in the port specification."); -1},
                };
                for p in start_range..=end_range {
                    if 0<p && p<65536 {
                    ports.push(p);
                    } else {println!("Error: Ports must be between 0 and 65,535. {}", p);}
                }
            } else {
                println!("Failed to parse input [{s}] due to error: {:?}", e);
            }
        },
        }
    }
    ports
}
