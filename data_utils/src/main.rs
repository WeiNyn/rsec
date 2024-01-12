use structopt::StructOpt;
use std::time::Instant;

mod command_options;
mod ip_parser;

use command_options::{Options, Subcommand};
use ip_parser::parse_ip;

fn main() {
    let options = Options::from_args();

    match options.subcommand {
        Subcommand::ParseIP(args) => {
            let now = Instant::now();
            parse_ip(&args.input, &args.output, &args.geolib, &args.separator).unwrap();
            println!("Elapsed time: {:?}", now.elapsed());
        }
    }
}
