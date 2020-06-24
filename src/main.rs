use tsunami::*;

fn main() {
    match core_dns::lookup_ns("www.dollarkiller.com".to_string()) {
        Err(e) => panic!(format!("err: {}",e)),
        Ok(msg) => {
            println!("{}",msg);
        },
    };


    println!("Hello, world!");
}
