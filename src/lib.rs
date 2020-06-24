pub mod config;
pub mod core_dns;

#[cfg(Test)]
mod test {
    use super::*;
    #[test]
    fn lookup_ns() {
        match core_dns::lookup_ns("www.dollarkiller.com") {
            Ok(_data) => {},
            Err(e) => panic!("err: ",e),
        };
    }
    #[test]
    fn lp() {
        let a = "www.dollarkiller.com";
        let b:Vec<&str> = a.split(".").collect();
        println!("b: {:?}",b);
    }
}