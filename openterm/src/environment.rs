"env" => {
    if args.len() < 2 {
        environment::help();
    } else {
        match args[1] {
            "help" => environment::help(),
println!("env                  Environment variables");

            "set" => {
                if args.len() > 3 {
                    environment::set(args[2], &args[3..].join(" "));
                } else {
                    println!("Usage: env set <key> <value>");
                }
            }

            "get" => {
                if args.len() > 2 {
                    environment::get(args[2]);
                } else {
                    println!("Usage: env get <key>");
                }
            }

            "unset" => {
                if args.len() > 2 {
                    environment::unset(args[2]);
                } else {
                    println!("Usage: env unset <key>");
                }
            }

            _ => println!("Unknown env command"),
        }
    }
}
