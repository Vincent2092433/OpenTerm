use std::process::Command;

pub fn run(program: &str, args: &[&str]) {
    let output = Command::new(program)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }

            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }

        Err(e) => {
            println!("Unknown command '{}': {}", program, e);
        }
    }
}