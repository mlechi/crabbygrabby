#![allow(unused)]
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use crate::ScanRequest;

pub fn connect_scan(req:ScanRequest){
    //First implementation should work with only one address and one port.
    /*let port = &req.ports[0];
    let address = &req.target_addresses[0];
    let target:String = format!("{}:{}", address, port.to_string());
    match TcpStream::connect(target){
        Ok(x) => println!("It worked!"),
        Err(x) => println!("It didn't work!"),
    }*/
    //Second implementation should work with one address and many ports.
    let address = &req.target_addresses[0];
    for p in req.ports {
        let target:String = format!("{}:{}", address, p.to_string());
        match TcpStream::connect(target){
        Ok(x) => println!("It worked! (port {})",p),
        Err(x) => println!("It didn't work! (port {})",p),
    }
    }
    //Third implementation should work with many addresses and many ports.
    //This won't work until after multi address spec is added in parse_args.rs
}

//THIS WAS mostly WRITTEN BY CHATGPT, NOT ME. I AM NOT TAKING CREDIT FOR THIS. THE ABOVE WAS WRITTEN BY ME, BUT THE BELOW WAS WRITTEN BY CHATGPT.
//I put it in the loop to scan multiple ports, and I made it work with the scanrequest struct.
//It doesn't really work as intended, so I'll work on something that does in the future.
pub fn syn_scan(req:ScanRequest){
    println!("SYN scan called!!!!!");
    let address = &req.target_addresses[0];
    for p in req.ports {
        let target: String = format!("{}:{}", address, p.to_string());
        match TcpStream::connect(target){
            Err(x) => println!("It didn't work! (port {})",p),
            Ok(x) => {
                println!("It worked! (port {})",p);
                let mut stream = x;
                //stream.set_write_timeout(Some(Duration::from_secs(1))).unwrap(); // Commenting this out doesn't prevent read from failing.
                stream.set_nonblocking(true).expect("set_non_blocking"); //This makes the program panic at stream.read.
                stream.write(&[0x02]).unwrap();
                let mut buf = [0; 1024];
                //This is a syn scan because stream.read is blocking. Until something is read, the program
                //does not continue, and does not send syn/ack.
                let size = match stream.read(&mut buf){
                    Err(x) => {println!("Error kind: {}",x.kind()); continue},
                    //Err(x) => break,
                    Ok (x)=> x,
                };
                if size > 0 && buf[0] == 0x12 {
                    println!("Port {} is open", p);
                }
            }
        }
    }
}
