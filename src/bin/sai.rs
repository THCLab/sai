const VERSION: &'static str = env!("CARGO_PKG_VERSION");

use clap::{App, Arg};
use said::derivation::SelfAddressing;
use std::str::FromStr;

fn main() {
    let matches = App::new("SAI")
        .version(VERSION)
        .subcommand(
            App::new("gen")
                .about("Generate Self-Addressing Identifier")
                .arg(
                    Arg::new("data")
                        .short('d')
                        .long("data")
                        .required(true)
                        .takes_value(true)
                        .help("Source data against which we would like toverify given digest"),
                )
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .takes_value(true)
                        .required(true)
                        .help(
                            "Derevation code for the digest, algorithm used for digest.
Supported codes:
   E - Blake3_256
   F - Blake2B256
   G - Blake2S256
   H - SHA3_256
   I - SHA2_256
   0D - Blake3_512
   0E - SHA3_512
   0F - Blake2B512
   0G - SHA2_512",
                        ),
                ),
        )
        .subcommand(
            App::new("verify")
                .about("Verify SAI with provided data")
                .arg(
                    Arg::new("sai")
                        .short('s')
                        .long("sai")
                        .takes_value(true)
                        .required(true)
                        .help("Digest against which we would like to verify the content"),
                )
                .arg(
                    Arg::new("data")
                        .short('d')
                        .long("data")
                        .takes_value(true)
                        .required(true)
                        .help("Source data against which we would like toverify given digest"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("gen") {
        let data = matches.value_of("data").unwrap().as_bytes();
        let prefix_code = matches.value_of("type").unwrap();
        let sai = SelfAddressing::from_str(prefix_code).unwrap();
        let calculated_sai = sai.derive(data).to_str();
        println!("{}", calculated_sai);
    }
    if let Some(matches) = matches.subcommand_matches("verify") {
        let data = matches.value_of("data").unwrap().as_bytes();
        let sai = matches.value_of("sai").unwrap();
        let prefix = SelfAddressing::from_str(sai).unwrap();
        let calculated_sai = prefix.derive(data).to_str();
        println!("{:?}", calculated_sai.eq(sai));
    }
}
