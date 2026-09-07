use std::process::Command;

pub fn help() {
    println!("===============================");
    println!(" OpenTerm Network Toolkit");
    println!("===============================");
    println!("network help");
    println!("network ping <host>");
    println!("network dns <host>");
    println!("===============================");
}

pub fn ping(host: &str) {
    let status = Command::new("ping")
        .arg("-c")
        .arg("4")
        .arg(host)
        .status();

    match status {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}

pub fn dns(host: &str) {
    let output = Command::new("nslookup")
        .arg(host)
        .output();

    match output {
        Ok(result) => {
            println!("{}", String::from_utf8_lossy(&result.stdout));
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
