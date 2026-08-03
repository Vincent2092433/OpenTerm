use std::collections::HashMap;
use std::io::{self, Write};
use std::fs;


pub fn run(script: &str) {

    let mut variables: HashMap<String, String> = HashMap::new();

    let mut functions: HashMap<String, Vec<String>> = HashMap::new();


    let lines: Vec<String> = script
        .lines()
        .map(|x| x.to_string())
        .collect();



    let mut index = 0;


    // Load functions

    while index < lines.len() {


        let line = lines[index].trim();


        if line.starts_with("function ") {


            let name = line
                .replace("function ", "")
                .trim()
                .to_string();


            let mut body = Vec::new();


            index += 1;


            while index < lines.len()
                && lines[index].trim() != "end"
            {

                body.push(
                    lines[index].clone()
                );

                index += 1;

            }


            functions.insert(
                name,
                body
            );

        }


        index += 1;

    }



    execute_lines(
        &lines,
        &mut variables,
        &functions
    );

}




fn execute_lines(
    lines: &Vec<String>,
    variables: &mut HashMap<String,String>,
    functions: &HashMap<String,Vec<String>>
) {


    let mut index = 0;



    while index < lines.len() {


        let line = lines[index].trim();


        if line.is_empty()
            || line.starts_with("function ")
            || line == "end"
        {

            index += 1;
            continue;

        }



        let parts: Vec<&str> =
            line.split_whitespace().collect();



        match parts[0] {



            "print" => {


                let mut output =
                    parts[1..].join(" ");



                for (key,value) in variables.iter() {


                    output = output.replace(
                        &format!("${}", key),
                        value
                    );

                }



                println!("{}", output);

            }





            "set" => {


                if parts.len() >= 3 {


                    variables.insert(
                        parts[1].to_string(),
                        parts[2..].join(" ")
                    );

                }

            }





            "input" => {


                if parts.len() > 1 {


                    print!("{}: ", parts[1]);

                    io::stdout()
                        .flush()
                        .unwrap();


                    let mut value = String::new();


                    io::stdin()
                        .read_line(&mut value)
                        .unwrap();



                    variables.insert(
                        parts[1].to_string(),
                        value.trim().to_string()
                    );

                }

            }





            "add" | "subtract" | "multiply" => {


                if parts.len() >= 4 {


                    let a =
                        variables
                        .get(parts[2])
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>()
                        .unwrap_or(0);



                    let b =
                        variables
                        .get(parts[3])
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>()
                        .unwrap_or(0);



                    let result = match parts[0] {


                        "add" => a + b,

                        "subtract" => a - b,

                        "multiply" => a * b,

                        _ => 0

                    };



                    variables.insert(
                        parts[1].to_string(),
                        result.to_string()
                    );

                }

            }





        "save" => {

    if parts.len() >= 3 {

        let filename = parts[1];

        let value = match variables.get(parts[2]) {
            Some(v) => v.clone(),
            None => String::new(),
        };

        match fs::write(filename, value) {
            Ok(_) => println!("Saved {}", filename),
            Err(e) => println!("Failed to save {}: {}", filename, e),
        };

    } else {

        println!("Usage: save <file> <variable>");

    }

}





            "load" => {


                if parts.len() >= 3 {


                    match fs::read_to_string(parts[1]) {


                        Ok(data) => {


                            variables.insert(
                                parts[2].to_string(),
                                data
                            );


                        }


                        Err(e) => {

                            println!("Error: {}", e);

                        }

                    }

                }

            }





            "call" => {


                if parts.len() > 1 {


                    if let Some(code) =
                        functions.get(parts[1])
                    {


                        execute_lines(
                            code,
                            variables,
                            functions
                        );


                    }


                }

            }





            "run" => {


                if parts.len() > 1 {


                    crate::commands::execute(
                        &parts[1..].join(" ")
                    );

                }

            }





            _ => {


                println!(
                    "Unknown SpellScript command: {}",
                    parts[0]
                );


            }

        }


        index += 1;

    }

}