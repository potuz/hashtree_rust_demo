extern crate pkg_config;

fn main() {
    match pkg_config::probe_library("hashtree") {
        Ok(_) => println!("Found hashtree."),
        Err(e) => panic!("hashtree not foudn: {:?}", e),
    }
}
