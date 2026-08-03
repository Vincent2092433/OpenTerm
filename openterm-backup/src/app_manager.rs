use std::fs;
use std::path::Path;

const APPS_DIR: &str = "apps";


pub fn help() {

    println!("===============================");
    println!(" SpellShark App Manager");
    println!("===============================");
    println!("app list");
    println!("app create <name>");
    println!("app remove <name>");
    println!("app info <name>");
    println!("app run <name>");
    println!("app help");

}



pub fn list() {

    if !Path::new(APPS_DIR).exists() {

        println!("No apps installed.");
        return;

    }


    println!("Installed Apps:");


    match fs::read_dir(APPS_DIR) {

        Ok(entries) => {

            for entry in entries.flatten() {

                if entry.path().is_dir() {

                    println!(
                        "- {}",
                        entry.file_name().to_string_lossy()
                    );

                }

            }

        }


        Err(e) => {

            println!("Error: {}", e);

        }

    }

}




pub fn create(name: &str) {


    let app_dir = format!("{}/{}", APPS_DIR, name);



    if Path::new(&app_dir).exists() {

        println!("App '{}' already exists.", name);
        return;

    }



    fs::create_dir_all(&app_dir).unwrap();



    let info = format!(
        "Name: {}\nVersion: 1.0.0\nAuthor: SpellShark\nDescription: {} application\n",
        name,
        name
    );



    fs::write(
        format!("{}/app.txt", app_dir),
        info
    )
    .unwrap();



    let script = format!(
r#"print Welcome to {}

set author SpellShark

print Created by $author

print App is running

run pwd

print Finished
"#,
        name
    );



    fs::write(
        format!("{}/main.spell", app_dir),
        script
    )
    .unwrap();



    println!(
        "App '{}' created successfully.",
        name
    );

}





pub fn remove(name: &str) {


    let app_dir = format!("{}/{}", APPS_DIR, name);



    if !Path::new(&app_dir).exists() {

        println!("App '{}' not found.", name);
        return;

    }



    fs::remove_dir_all(&app_dir).unwrap();



    println!(
        "App '{}' removed.",
        name
    );

}




pub fn info(name: &str) {


    let info_file = format!(
        "{}/{}/app.txt",
        APPS_DIR,
        name
    );



    if !Path::new(&info_file).exists() {

        println!(
            "App '{}' not found.",
            name
        );

        return;

    }



    match fs::read_to_string(info_file) {


        Ok(data) => {

            println!("{}", data);

        }


        Err(e) => {

            println!("Error: {}", e);

        }

    }

}





pub fn run(name: &str) {


    let spell_file = format!(
        "{}/{}/main.spell",
        APPS_DIR,
        name
    );



    if !Path::new(&spell_file).exists() {


        println!(
            "App '{}' does not have main.spell",
            name
        );


        return;

    }



    println!("===============================");
    println!("Running SpellScript app: {}", name);
    println!("===============================");



    match fs::read_to_string(spell_file) {


        Ok(script) => {


            crate::spellscript::run(&script);


        }


        Err(e) => {


            println!(
                "Error loading app: {}",
                e
            );


        }

    }

}