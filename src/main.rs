#![allow(unused_imports)]
#![allow(dead_code)]
use std::{env, net::TcpStream};
mod parse_args;
mod scan;
//use parse_args::parse;
fn main() {
  //returns a ScanRequest.
  let scan_req = parse_args::parse();
  scan_req.perform_scan();
}
#[derive(Debug)]
#[allow(unused)]
pub struct ScanRequest{
  //With -p flag, just add the specified ports. Without -p flag, add all 65,535 ports.
  ports: Vec<i32>,
  //for now, one address at a time. manually parsing ip addresses and subnets seems like it will be a pain,
  //and is not the point of this excercise anyway.
  // Nevertheless, the option remains to add this functionality (In parse_args.rs) later.
  target_addresses: Vec<String>,
  scan_type: ScanType,
}
impl ScanRequest{
  fn perform_scan(self){
    match self.scan_type{
      ScanType::Normal => scan::connect_scan(self),
      ScanType::Syn => scan::syn_scan(self),
      ScanType::NoScan => (),
    }
  }
}
#[derive(Debug)]
enum ScanType{Normal, Syn, NoScan,}
