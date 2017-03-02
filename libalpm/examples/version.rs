extern crate libalpm;

fn main() {
    println!("Version: {}", libalpm::version());
    println!("Capabilities: {:?}", libalpm::capabilities());
}


