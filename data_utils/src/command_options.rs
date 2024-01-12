use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(name = "parse-ip", about= "Parse IP addresses")]
    ParseIP(ParseIPArguments),
}

#[derive(Debug, StructOpt)]
pub struct ParseIPArguments {
    #[structopt(short = "i", long = "input", help = "Input file")]
    pub input: String,

    #[structopt(short = "o", long = "output", help = "Output file")]
    pub output: String,

    #[structopt(short = "g", long = "geolib", help = "Geo library", default_value = "data/GeoLite2-City.mmdb")]
    pub geolib: String,

    #[structopt(short = "s", long = "separator", help = "Separator", default_value = ",")]
    pub separator: String,
}