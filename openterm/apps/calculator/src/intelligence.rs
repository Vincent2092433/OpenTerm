pub fn help() {
    println!("================ OpenTerm Intelligence ================");
    println!("intel help              Show intelligence commands");
    println!("intel domain <name>     Analyze domain");
    println!("intel ip <address>      Analyze IP address");
    println!("=======================================================");
}


pub fn domain(name: &str) {
    println!("Domain analysis:");
    println!("Target: {}", name);
}


pub fn ip(address: &str) {
    println!("IP analysis:");
    println!("Target: {}", address);
}
