use std::io;
use std::io::Write;
use std::process;

use vedic_core::aadhaar::Aadhaar;
use vedic_core::dosasucana::Dosa;
use vedic_core::fetchsource::Sourcecode;
// use vedic_core::debug;

pub fn help() {
    println!(
        "
    Vish Language v{}
    
    A Sanskrit-inspired programming language created by Vishesh.
    
    Developed with ❤️ by: Vishesh

    Usage: vish [--option] [path/to/script.vish]

    Options:
    -h, --help      Show this help message
    -v, --version   Print version number
    -r, --repl      Run the interactive REPL

    Supported Extensions: .vish / .ved / .veda / .vedic

    Examples:
    ➤ Run a Vish script
      vish script.vish

    ➤ Start the REPL
      vish -r

    ➤ Math operations
      मान x = मूल(16) + घात(2, 3);
      वद(x);  # Output: 12
",
        env!("CARGO_PKG_VERSION")
    );
}


// ❯ this command will install the panchang.ved vedic script from given url
// vedic -i https://git.com/panchang.ved

// pub fn format(_args: Vec<String>) {
//     println!("Format Feature is not available yet.");
//     process::exit(0);
// }

// pub fn install(_args: Vec<String>) {
//     println!("Install Feature is not available yet.");
//     process::exit(0);
// }

pub fn version() {
    println!("Vedic v{}", env!("CARGO_PKG_VERSION"));
}

pub fn run(args: Vec<String>) {
    if args[1].starts_with('-') {
        eprintln!("Invalid argument.");
    } else {
        run_file(&args[1]);
    }
}

pub fn run_file(path: &str) {
    // Support .vish extension for Vishesh's custom branding!
    if path.ends_with(".ved") || path.ends_with(".veda") || path.ends_with(".vedic") || path.ends_with(".vish") {
        let mut aadhaar = Aadhaar::new();
        aadhaar.prarambha();
        let sc = match Sourcecode::new(path) {
            Ok(content) => content,
            Err(dosa) => {
                eprintln!("Unable to read file : {dosa}");
                process::exit(0);
            }
        };

        if let Err(dosa) = aadhaar.vyakhyati(sc) {
            match dosa {
                Dosa::SagkalanaKaleDosa => process::exit(65),
                Dosa::AnusthanaKaleDosa => process::exit(70),
            }
        }
    } else {
        eprintln!("Invalid file extension. Supported: .ved / .veda / .vedic / .vish");
        process::exit(0);
    }
}


pub fn repl() {
    let mut aadhaar = Aadhaar::new();
    println!(
        "Welcome to Vedic v{}
REPL session

❯ Type #निर्गम or #exit to Exit the REPL.
",
        env!("CARGO_PKG_VERSION")
    );
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Unable to read line from the REPL");
        match line.trim() {
            "#निर्गम" | "#exit" => {
                println!("Exiting the REPL...");
                break;
            }
            "" => continue,
            _ => {
                aadhaar
                    .vyakhyati(Sourcecode {
                        code: line,
                        path: "script",
                    })
                    .ok();
            }
        }
    }
}
