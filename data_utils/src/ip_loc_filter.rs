use failure::Error;

use polars::lazy::dsl::col;
use polars::prelude::*;

use std::path::PathBuf;

pub fn filter_ip_loc(
    cur_path: &str,
    pass_path: Vec<&str>,
    output: &str,
) -> Result<(), Error> {
    let mut scan_parquet_args = ScanArgsParquet::default();
    scan_parquet_args.low_memory = true;

    let cur_path = cur_path.to_string();
    let pass_path: Arc<[PathBuf]> = pass_path
        .iter()
        .map(|p| PathBuf::from(p))
        .collect::<Vec<_>>()
        .into();

    let cur_df = LazyFrame::scan_parquet(cur_path, scan_parquet_args.clone())
        .expect("Cannot read cur file")
        .filter(col("loc").eq(lit("KH")))
        .unique(Some(vec![String::from("uid")]), UniqueKeepStrategy::First);
    let pass_df = LazyFrame::scan_parquet_files(pass_path, scan_parquet_args)
        .expect("Cannot read pass file")
        .filter(col("loc").eq(lit("VN")))
        .select(&[col("uid")])
        .unique(Some(vec![String::from("uid")]), UniqueKeepStrategy::First);

    let mut serialize_options = SerializeOptions::default();
    serialize_options.separator = b'\t';
    
    let mut write_options = CsvWriterOptions::default();

    write_options.include_header = true;
    write_options.serialize_options = serialize_options;

    cur_df
        .join(
            pass_df,
            [col("uid")],
            [col("uid")],
            JoinArgs::new(JoinType::Inner),
        )
        .sink_parquet(PathBuf::from(output), ParquetWriteOptions::default())
        .expect("Cannot write file");

    Ok(())
}
