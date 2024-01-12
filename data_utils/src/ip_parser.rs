use failure::Error;

use maxminddb::geoip2;

use polars::datatypes::DataType::String as utf8;
use polars::lazy::dsl::{col, GetOutput};
use polars::prelude::*;

use smartstring::SmartString;

use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;

pub fn parse_ip(input: &str, output: &str, geolib: &str, separator: &str) -> Result<(), Error> {
    let mut schema = Schema::new();
    schema.with_column(SmartString::from("ts"), DataType::UInt64);
    schema.with_column(SmartString::from("uid"), DataType::UInt64);
    schema.with_column(SmartString::from("ip"), utf8);

    let geolib = geolib.to_string();
    LazyCsvReader::new(input)
        .has_header(false)
        .with_separator(separator.as_bytes()[0])
        .with_schema(Some(Arc::new(schema)))
        .finish()
        .expect("Cannot read file")
        .select(&[
            col("ts"),
            col("uid"),
            col("ip"),
            col("ip")
                .map(
                    move |s: Series| {
                        let reader =
                            maxminddb::Reader::open_readfile(&geolib).expect("Cannot read geolib");
                        let out = s
                            .str()
                            .expect("Cannot convert to string")
                            .into_iter()
                            .map(|ip| match ip {
                                Some(ip) => {
                                    let city = reader
                                        .lookup::<geoip2::City>(IpAddr::from_str(&ip).unwrap())
                                        .unwrap();
                                    match city.country {
                                        Some(country) => country.iso_code.unwrap().to_string(),
                                        None => String::from(""),
                                    }
                                }
                                _ => String::from(""),
                            })
                            .collect();
                        Ok(Some(out))
                    },
                    GetOutput::from_type(utf8),
                )
                .alias("loc"),
        ])
        .sink_parquet(PathBuf::from(output), ParquetWriteOptions::default())
        .expect("Cannot write to file");

    Ok(())
}
