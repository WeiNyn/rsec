use structopt::StructOpt;
use std::path::PathBuf;
use std::time::Instant;
use chrono::prelude::*;
use chrono::Duration;
use kdam::tqdm;

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
        },
        Subcommand::BackfillIP(args) => {
            println!("Backfill IP");
            let from_date = NaiveDate::parse_from_str(&args.from, "%y-%m-%d").unwrap();
            let to_date = NaiveDate::parse_from_str(&args.to, "%y-%m-%d").unwrap();

            let duration = to_date.signed_duration_since(from_date);
            let days = duration.num_days();

            println!("Backfill for {} days from {:?} to {:?}", days, from_date, to_date);

            for i in tqdm!(0..days) {
                let date = from_date + Duration::days(i);
                let input = args.input.replace("{date}", &date.format("%y-%m-%d").to_string());
                let output = args.output.replace("{date}", &date.format("%y-%m-%d").to_string());

                let output_path = PathBuf::from(output.clone());
                let output_folder = output_path.parent().unwrap();
                if !output_folder.exists() {
                    std::fs::create_dir_all(output_folder).unwrap();
                }

                println!("Input: {}", input);
                println!("Output: {}", output);
                parse_ip(&input, &output, &args.geolib, &args.separator).unwrap();
            }

        },
    }
}
