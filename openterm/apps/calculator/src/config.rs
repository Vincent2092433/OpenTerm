use std::fs;

const CONFIG_FILE: &str = "openterm_config.txt";


pub fn set(key: &str, value: &str) {

    let mut config = load();


    config.retain(|line| {
        !line.starts_with(&format!("{}=", key))
    });


    config.push(format!("{}={}", key, value));


    fs::write(
        CONFIG_FILE,
        config.join("\n")
    )
    .unwrap();


    println!("Config updated: {} = {}", key, value);
}



pub fn get_value(key: &str) -> String {

    let config = load();


    for line in config {

        if line.starts_with(&format!("{}=", key)) {

            return line
                .split('=')
                .nth(1)
                .unwrap_or("")
                .to_string();

        }

    }


    String::new()
}



pub fn get(key: &str) {

    let value = get_value(key);


    if value.is_empty() {

        println!("No configuration found for '{}'", key);

    } else {

        println!("{} = {}", key, value);

    }

}



pub fn list() {

    let config = load();


    if config.is_empty() {

        println!("No configuration set.");

    } else {

        println!("OpenTerm Configuration:");

        for item in config {

            println!("{}", item);

        }

    }

}



fn load() -> Vec<String> {

    fs::read_to_string(CONFIG_FILE)
        .unwrap_or_default()
        .lines()
        .map(|x| x.to_string())
        .collect()

}