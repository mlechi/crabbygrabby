#![allow(unused_imports)]
#![allow(dead_code)]
use std::{env, net::{TcpStream, SocketAddrV4}};
//mod parse_args;
mod scan;
mod ip_parser;
fn main() {
  let arguments: Vec<String> = env::args().collect();
  let scan = ScanRequest {targets: ip_parser::parse_socket_range(&arguments).unwrap(), scan_type: ScanType::Normal};
  //println!("{:?}", scan);
  scan.perform_scan();
}
#[derive(Debug)]
#[allow(unused)]
pub struct ScanRequest{
  //With -p flag, just add the specified ports. Without -p flag, add all 65,535 ports.
  //ports: Vec<i32>,
  //With -t flag, the following addresses. Without, default to loopback address.
  //target_addresses: Vec<String>,
  targets: Vec<SocketAddrV4>,
  scan_type: ScanType,
}
impl ScanRequest{
  fn perform_scan(self){
    match self.scan_type{
      ScanType::Normal => scan::connect_scan(self),
      ScanType::Syn => (),//scan::syn_scan(self),
      ScanType::NoScan => (),
    }
  }
}
#[derive(Debug)]
enum ScanType{Normal, Syn, NoScan,}
