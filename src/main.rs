// use queue_file::QueueFile;
use std::env::args;

struct Args {
    file: String,
}

struct ParseViolation {
    root: String,
    message: String,
}

struct ParseError {
    violations: Vec<ParseViolation>,
}

fn main() {
    match parse_args() {
        Ok(args) => println!("{}", args.file),
        Err(errors) => {
            for violation in errors.violations {
                println!("Error: {} at `{}`", violation.message, violation.root);
            }
            println!("");
            println!("USAGE:");
            println!("  hello-queue-file <file>");
        }
    };
}

fn parse_args() -> Result<Args, ParseError> {
    struct Parsed {
        file: Option<String>,
    }
    let file = args().nth(1).or(None);
    let parsed = Parsed {
        file: file.or(None),
    };

    let mut violations = vec![];
    if parsed.file == None {
        violations.push(ParseViolation {
            root: "file".to_string(),
            message: format!("No {} specified", "file"),
        });
    }

    if violations.len() > 0 {
        return Err(ParseError { violations });
    }

    Ok(Args {
        file: parsed.file.unwrap(),
    })
}
