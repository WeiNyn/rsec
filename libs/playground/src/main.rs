use polars::{prelude::*, lazy::dsl::{col, lit}};

fn main() {
    let s = Series::new("a", ["1", "2", "3", "4", "5"]);
    let df = DataFrame::new(vec![s]).unwrap();
    let b = df.clone().lazy().select([col("a").eq(lit("1")).or(col("a").eq(lit("2")))]).collect().unwrap();
    println!("{:?}", b);
}
