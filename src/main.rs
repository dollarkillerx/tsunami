use tsunami::*;

fn main() {
    match core_dns::lookup_ns("dollarkiller.com".to_string()) {
        Err(e) => panic!(format!("err: {}",e)),
        Ok(msg) => {
            // ns parse
            match core_dns::lookup_a(msg) {
                Err(e) => {
                    panic!(format!("{}",e));
                },
                Ok(a) => {
                    println!("a: {:?}",a);
                    // ddos start

                },
            }
        },
    };


    println!("Hello, world!");
}
