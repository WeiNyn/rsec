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

    #[structopt(name = "backfill-ip", about="Backfill IP addresses")]
    BackfillIP(BackfillIPArguments),
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

#[derive(Debug, StructOpt)]
pub struct BackfillIPArguments {
    #[structopt(short = "i", long = "input", help = "Input schema")]
    pub input: String,

    #[structopt(short = "o", long = "output", help = "Output schema")]
    pub output: String,

    #[structopt(short = "f", long = "from", help = "From date")]
    pub from: String,

    #[structopt(short = "t", long = "to", help = "To date")]
    pub to: String,

    #[structopt(short = "g", long = "geolib", help = "Geo library", default_value = "data/GeoLite2-City.mmdb")]
    pub geolib: String,

    #[structopt(short = "s", long = "separator", help = "Separator", default_value = ",")]
    pub separator: String,
}