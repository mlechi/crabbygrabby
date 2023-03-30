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
  //With -t flag, the following addresses. Without, default to loopback address.
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
