use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(about, version, arg_required_else_help(true))]
/// Reads JSON5 files and print them to stdout as plain old JSON.
struct Cmd {
    #[arg(short, long, help = "Pretty-print JSON output")]
    pretty: bool,

    #[arg(num_args(1..), help = "Files to load JSON5 data from")]
    files: Vec<String>,
}

fn main() {
    let Cmd { pretty, files } = Cmd::parse();

    let mut failed = false;
    for file in files {
        let data = match read(&*file) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to read file: {}: {}", file, error);
                failed = true;
                continue;
            }
        };

        let value = match json5::from_str::<serde_json::Value>(&data) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Failed to parse JSON: {}: {}", file, error);
                failed = true;
                continue;
            }
        };

        if pretty {
            println!("{:#}", value);
        } else {
            println!("{}", value);
        }
    }

    if failed {
        std::process::exit(1);
    }
}

fn read(file: &str) -> Result<String, std::io::Error> {
    if file == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read_to_string(&file)
    }
}
