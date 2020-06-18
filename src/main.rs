use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    HexToBase64 { hextext: String },
    FixedXOR { first: String, second: String },
}

fn main() {
    let args = Cli::from_args();

    let output = match args {
        Cli::HexToBase64 { hextext } => cryptopals::hex_to_base64(hextext),
        Cli::FixedXOR { first, second } => cryptopals::fixed_xor(first, second),
    };

    match output {
        Ok(output) => println!("{}", output),
        Err(error) => println!("An error occurred: {}", error),
    }
}
