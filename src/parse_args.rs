#![allow(unused)]
use crate::{ScanRequest, ScanType};
use std::{env, str::FromStr};

pub fn parse()-> ScanRequest{
    //Collect args. In future, consider passing args: Vec<String> in as a parameter.
    let args:Vec<String> = env::args().collect();
    //Below are the indexes of the args for ports and addresses. They will always be the args preceded by -p and -t.
    let mut arg_index_port:usize = 0;
    let mut arg_index_port_end:usize = 0;
    let mut arg_index_address:usize = 0;
    let mut arg_index_address_end:usize = 0;
    //These are the port and address vectors
    let mut prt:Vec<i32> = (1..=65535).collect();
    let mut addrs:Vec<String> = vec!["127.0.0.1".to_string()];
    //Below is the index of the scan type. Always preceded by -st.
    let mut arg_index_scan_type:u8 = 0;
    //Initialize the ScanType
    let mut s_t: ScanType = ScanType::Normal;
    //Find the indexes.
    //println!("Arguments given: {:?}",args);
    if args.len() > 1 {
        for i in 0..args.len(){
            match args[i].as_str(){
                //port
                "-p" => {
                    arg_index_port = i+1;
                    arg_index_port_end = arg_index_port + 1;
                    for i in &args[arg_index_port..] {
                        if i.ends_with(",") {
                            arg_index_port_end +=1;
                        } else { continue; }
                    }
                    //prt = ports_parse(&args[i+1]);
                    prt = match ports_parse(&args[arg_index_port..arg_index_port_end].to_vec()) {
                        Ok(x) => x,
                        Err(_) => vec![0],
                    }
                },
                //ipv4 target
                "-t" => {
                    arg_index_address = i+1;
                    arg_index_address_end = arg_index_address + 1;
                    for i in &args[arg_index_address..] {
                        if i.ends_with(',') {
                            arg_index_address_end += 1;
                        } else {continue;}
                    }
                    addrs = match address_parse(&args[arg_index_address..arg_index_address_end].to_vec()) {
                        Ok(x) => x,
                        Err(_) => vec!["".to_string()],
                    };
                },
                //Scan type
                "-st" => {match args[i+1].as_str(){
                    "c" => s_t = ScanType::Normal,
                    "s" => s_t = ScanType::Syn,
                    _ => s_t = ScanType::Normal,
                }},
                _  => (),
            }
        }
        ScanRequest { ports: (prt), target_addresses: (addrs), scan_type: (s_t), }
    } else {
        help_message()
    }
}

//IPv4 addresses only
fn address_parse(input: &Vec<String>)->Result<Vec<String>, String>{
    let mut output:Vec<String> = Vec::new();
    //println!("address_parse input: {:?}",input);
    for address in input {
        let mut octet_number: u16 = 0;
        let mut octet_buffer:String = String::new();
        let mut add_buffer:String = String::new();
        let mut num_p: u8 = 0;
        let mut oct_range_start: String = String::from_str("").unwrap();
        for i in address.chars() {
            if i.is_numeric() {octet_buffer.push(i);}
            else if i == '.' {
                match &octet_buffer.parse::<u8>() {
                    Ok(x)  => {
                        octet_buffer.push(i);
                        if octet_buffer.len() > 2 {
                            while octet_buffer.starts_with("0") {
                                octet_buffer = octet_buffer[1..].to_string();
                            }
                        }
                        add_buffer.push_str(octet_buffer.as_str());
                    },
                    Err(_)  => {
                        octet_buffer.push(i);
                        println!("PERIOD An octet in an ip address failed to parse to a u8. add_buffer: {} octet_buffer: {}", add_buffer, octet_buffer);
                        add_buffer.clear();
                        num_p = 0;
                        octet_number = 0;
                    },
                }
                num_p += 1;
                octet_buffer.clear();
                octet_number += 1;
            }
            //When ranges and subnets are supported, this will be uncommented.
            //else if octet_number == 3 && (i == '-' || i == '/') {octet_buffer.push(i);}
            else if i == '-' {
                if octet_number != 3 {
                    //println!("Dashes are only supported in the last octet right now, to denote ranges.");
                    panic!("Dashes are only supported in the last octet right now, to denote ranges.");
                } else {
                    oct_range_start = octet_buffer.clone();
                    octet_buffer.clear();
                }
            }
            else if octet_number == 3 && i == ',' {
                match &octet_buffer.parse::<u8>() {
                    Ok(x)  => {
                        if octet_buffer.len() < 1 {
                            while octet_buffer.starts_with("0") {
                                octet_buffer = octet_buffer[1..].to_string();
                            }
                        }
                        num_p = 0;
                        if oct_range_start == "".to_string() {
                            add_buffer.push_str(octet_buffer.as_str());
                            output.push(add_buffer.clone());
                        } else {
                            for s in oct_range_start.parse::<u8>().unwrap()..=*x {
                                let add_address:String = format!("{}{}", add_buffer, s.to_string());
                                output.push(add_address);
                            }
                            oct_range_start = "".to_string();
                        }
                    },
                    Err(_)  => {println!("COMMA An octet in an ip address failed to parse to a u8."); add_buffer.clear();},
                }
                add_buffer.clear();
                octet_buffer.clear();
                octet_number = 0;
            }
            else if i == ',' {add_buffer.clear(); octet_buffer.clear(); octet_number = 0; num_p = 0;}
            else if octet_number > 3 {octet_buffer.clear(); add_buffer.clear();}
            else {add_buffer.clear(); octet_buffer.clear();}
        }
        if num_p > 3 && add_buffer != "" {
            println!("{} is Invalid IP address: Too many periods.", add_buffer);
            add_buffer.clear();
        } else if num_p < 3 && add_buffer != "" {
            println!("{} is Invalid IP address: Not enough periods.", add_buffer);
            add_buffer.clear();
        }
        if add_buffer != "" && octet_number >= 3 {
            match &octet_buffer.parse::<u8>() {
                Ok(x)  => {
                    if octet_buffer.len() < 1 {
                        while octet_buffer.starts_with("0") {
                            octet_buffer = octet_buffer[1..].to_string();
                        }
                    }
                    num_p = 0;
                    if oct_range_start == "".to_string() {
                        add_buffer.push_str(octet_buffer.as_str());
                        output.push(add_buffer.clone());
                    } else {
                        for s in oct_range_start.parse::<u8>().unwrap()..=*x {
                            let add_address:String = format!("{}{}", add_buffer, s.to_string());
                            output.push(add_address);
                        }
                    }
                },
                Err(_)  => {println!("COMMA An octet in an ip address failed to parse to a u8."); add_buffer.clear();},
            }
        };
    }
    //println!("address_parse output: {:?}",output);
    Ok(output)
}

//There absolutely must be a more efficient way to do this.
fn ports_parse(string_list:&Vec<String>)->Result<Vec<i32>, String>{
    //Called on arg preceded by -p.
    let mut output:Vec<i32> = Vec::new();
    for list in string_list {
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
        //} else if !i.is_numeric(){
        } else {
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
    output.append(&mut ports);
    }
    Ok(output)
}

fn help_message() -> ScanRequest {
    println!("");
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
    println!("");
    ScanRequest { ports: (vec![0]), target_addresses: (vec!["".to_string()]), scan_type: (ScanType::NoScan) }
}
