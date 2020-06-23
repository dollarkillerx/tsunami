use tsunami::*;

fn main() {
    match core_dns::lookup_ns("www.dollarkiller.com") {
        Err(e) => panic!(format!("err: {}",e)),
        Ok(_) => {},
    };
    println!("Hello, world!");
}
