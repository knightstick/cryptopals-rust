use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Cli {
    HexToBase64 { hextext: String },
}

fn main() {
    let args = Cli::from_args();

    let output = match args {
        Cli::HexToBase64 { hextext } => cryptopals::hex_to_base64(hextext),
    };

    println!("{}", output)
}

// 0100 1001 0010
// 010010 010010
