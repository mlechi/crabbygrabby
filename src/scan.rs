use std::io::ErrorKind;
//#![allow(unused)]
use std::net::{TcpStream, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use crate::ScanRequest;

pub fn connect_scan(req:ScanRequest){
    println!("connect_scan called!");
    //println!("{:?}", req.targets);
    let mut refused: Vec<SocketAddr> = Vec::new();
    //let mut timed_out: Vec<SocketAddr> = Vec::new();
    for target in &req.targets {
        let t = SocketAddr::from(*target);
        match TcpStream::connect_timeout(&t, Duration::from_secs(1)) {
            Ok(_) => println!("    {}: Open",t),
            //Ok(_) => println!(""),
            //Err(_) => println!("    {}: Closed",p),
            //Err(x) => println!("Error: {}", x),
            Err(x) => if x.kind() == ErrorKind::ConnectionRefused {
                refused.push(t);
            } else if x.kind() == ErrorKind::TimedOut {
                //timed_out.push(t)
                //println!("    {}: Timed Out",t);
            },
        }
    }
    //println!("Refused: {:?}", refused);
    //println!("Timed out: {:?}", timed_out);
}
