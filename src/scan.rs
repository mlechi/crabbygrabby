#![allow(unused)]
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use crate::ScanRequest;

pub fn connect_scan(req:ScanRequest){
    println!("{:?}",req.target_addresses);
    for a in req.target_addresses {
        println!("Scanning: {}", a);
        if a == "".to_string() {
            //println!("Not Performing Scan.");
            continue;
        } else {
            for p in &req.ports {
                let target:String = format!("{}:{}", a, p.to_string());
                //match TcpStream::connect_timeout(&target.to_socket_addrs().expect("Not valid socket_addrs").next().expect("KILL ME"), Duration::from_secs(1)){
                match TcpStream::connect(target){
                    Ok(x) => println!("    Port Open: ({}:{})",a,p),
                    Err(x) => println!("    Port Closed: ({}:{})",a,p),
                }
            }
        }
    }
}

//This does not work as intended, and is not being worked on right now.
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
