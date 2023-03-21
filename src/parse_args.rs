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
    println!("Arguments given: {:?}",args);
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
}

fn address_parse(input: &String)->Vec<String>{
    //Placeholder that works on only one string, and does no input validation.
    vec![input.to_string()]
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
