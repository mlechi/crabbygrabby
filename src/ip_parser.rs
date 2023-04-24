#![allow(unused)]
use std::{env, net::{SocketAddrV4, Ipv4Addr}};

pub fn parse_socket_range(arguments: &Vec<String>) -> Result<Vec<SocketAddrV4>, String> {
    //To be used ONLY for the use case with multiple sockets, for now.
    //Receive full list of args.
    //Construct Vec<String> for args to send to parse_ports and parse_ip
    //Defaults to 127.0.0.1 and port 0
    let mut arg_index_port:usize = 0;
    let mut arg_index_port_end:usize = 0;
    let mut arg_index_address:usize = 0;
    let mut arg_index_address_end:usize = 0;
    let mut prt:Vec<u16> = vec![0];
    let mut addrs:Vec<Ipv4Addr> = vec![Ipv4Addr::new(127,0,0,1)];
    if arguments.len() > 1 {
        for i in 0..arguments.len(){
            match arguments[i].as_str(){
                //port
                "-p" => {
                    arg_index_port = i+1;
                    arg_index_port_end = arg_index_port;
                    for i in &arguments[arg_index_port..] {
                        if i.ends_with(",") {
                            arg_index_port_end +=1;
                        } else { break; }
                    }
                    prt = match ports_parse(&arguments[arg_index_port..=arg_index_port_end].to_vec()) {
                        Ok(x) => x,
                        Err(_) => vec![0],
                    }
                },
                //ipv4 target
                "-t" => {
                    arg_index_address = i+1;
                    arg_index_address_end = arg_index_address;
                    for i in &arguments[arg_index_address..] {
                        if i.ends_with(',') {
                            arg_index_address_end += 1;
                        } else {break;}
                    }
                    addrs = match ip_parse(&arguments[arg_index_address..=arg_index_address_end].to_vec()) {
                        Ok(x) => x,
                        Err(_) => vec![Ipv4Addr::new(127,0,0,1)],
                    };
                },
                _ => (),
            }
        }
    } else {
        println!("No Args Given.");
    }
    let mut output_sockets: Vec<SocketAddrV4> = Vec::new();
    //println!("addrs: {:?}", addrs);
    //println!("prt: {:?}", prt);
    for a in &addrs {
        for p in &prt {
            output_sockets.push(SocketAddrV4::new(*a, *p));
        }
    }
    Ok(output_sockets)
}

fn ports_parse(input: &Vec<String>) -> Result<Vec<u16>, String> {
    let mut output_vec: Vec<u16> = Vec::new();
    for p in input {
        let buff = p.split(",");
        for i in buff {
            if !i.contains("-") {
                let port_num: u16 = match i.parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => 0,
                };
                if port_num !=0 {output_vec.push(port_num);}
            } else {
                let mut port_split = i.split("-");
                let start_range = port_split.next().unwrap().parse::<u16>().unwrap();
                let end_range = match port_split.next().unwrap().parse::<u16>(){
                    Ok(res) => res,
                    Err(err) => {println!("You have a hanging dash in the port specification."); 0},
                };
                for x in start_range..=end_range {
                    output_vec.push(x);
                }
            }
        }
    }
    Ok(output_vec)
}

fn ip_parse(input: &Vec<String>) -> Result<Vec<Ipv4Addr>, String> {
    let mut output_vec: Vec<Ipv4Addr> = Vec::new();
    let parse_oct = |x: &str| {
        match x.parse::<u8>() {
            Ok(x) => x,
            Err(_) => {println!("{x} could not be parsed to a u8."); 0},
        }
    };
    for buffer in input {
        let addr_string = buffer.split(",");
        for addr in addr_string {
            if addr != "" {
                let mut addr_iter = addr.split(".");
                let oct1: u8 = parse_oct(addr_iter.next().unwrap());
                let oct2: u8 = parse_oct(addr_iter.next().unwrap());
                let oct3: u8 = parse_oct(addr_iter.next().unwrap());
                let oct4_s = addr_iter.next().unwrap();
                if oct4_s.contains("-") {
                    let mut add_split = oct4_s.split("-");
                    let start_range = add_split.next().unwrap().parse::<u8>().unwrap();
                    let end_range = match add_split.next().unwrap().parse::<u8>(){
                        Ok(res) => res,
                        Err(err) => {println!("You have a hanging dash in the port specification."); 0},
                    };
                    for x in start_range..=end_range {
                        output_vec.push(Ipv4Addr::new(oct1, oct2, oct3, x));
                    }
                } else {
                let oct4: u8 = parse_oct(oct4_s);
                output_vec.push(Ipv4Addr::new(oct1, oct2, oct3, oct4));
                }
            }
        }
    }
    Ok(output_vec)
}
