use polars::lazy::dsl::{col, lit};
use polars::prelude::*;
use polars_core::{chunked_array::ChunkedArray, datatypes::BooleanType, frame::DataFrame};
use serde::{Deserialize, Serialize};

trait Check<T: PartialEq> {
    fn check(&self, value: T) -> bool;
}

trait Predict {
    fn predict(&self, value: DataFrame) -> ChunkedArray<BooleanType>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum RuleType {
    Continuous,
    Discrete,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Range {
    min: f32,
    max: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ContinuousRule {
    name: String,
    rule_type: RuleType,
    ranges: Vec<Range>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DiscreteRule<T: PartialEq> {
    name: String,
    rule_type: RuleType,
    valid: Vec<T>,
    invalid: Vec<T>,
    space: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Rule<T: PartialEq> {
    Continuous(ContinuousRule),
    Discrete(DiscreteRule<T>),
}

impl Check<f32> for ContinuousRule {
    fn check(&self, value: f32) -> bool {
        for range in &self.ranges {
            if value > range.min && value <= range.max {
                return true;
            }
        }
        false
    }
}

impl Predict for ContinuousRule {
    fn predict(&self, value: DataFrame) -> ChunkedArray<BooleanType> {
        let dtype = value.column(&self.name).unwrap().dtype();
        match dtype {
            DataType::Float32 | DataType::Float64 => {
                let mut expression = lit(false);
                for range in &self.ranges {
                    expression = expression.or(col(&self.name)
                        .gt_eq(lit(range.min))
                        .and(col(&self.name).lt_eq(lit(range.max))));
                }
                value
                    .clone()
                    .lazy()
                    .select([expression])
                    .collect()
                    .unwrap()
                    .column("literal")
                    .unwrap()
                    .bool()
                    .unwrap()
                    .to_owned()
            }
            _ => panic!("Column type {:#?} is not Float", dtype),
        }
    }
}

impl<T: PartialEq> Check<T> for DiscreteRule<T> {
    fn check(&self, value: T) -> bool {
        if self.valid.contains(&value) {
            return true;
        }
        if self.invalid.contains(&value) {
            return false;
        }
        if self.space.contains(&value) {
            return false;
        }
        false
    }
}

impl Predict for DiscreteRule<String> {
    fn predict(&self, value: DataFrame) -> ChunkedArray<BooleanType> {
        let dtype = value.column(&self.name).unwrap().dtype();
        match dtype {
            DataType::String => {
                let mut expression = lit(false);
                for valid in &self.valid {
                    expression = expression.or(col(&self.name).eq(lit(valid.to_string())));
                }
                for invalid in &self.invalid {
                    expression = expression.and(col(&self.name).neq(lit(invalid.to_string())));
                }
                value
                    .clone()
                    .lazy()
                    .select([expression])
                    .collect()
                    .unwrap()
                    .column("literal")
                    .unwrap()
                    .bool()
                    .unwrap()
                    .to_owned()
            }
            _ => panic!("Column type {:#?} is not String", dtype),
        }
    }
}

impl Predict for DiscreteRule<i32> {
    fn predict(&self, value: DataFrame) -> ChunkedArray<BooleanType> {
        let dtype = value.column(&self.name).unwrap().dtype();
        match dtype {
            DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => {
                let mut expression = lit(false);
                for valid in &self.valid {
                    expression = expression.or(col(&self.name).eq(lit(*valid)));
                }
                for invalid in &self.invalid {
                    expression = expression.and(col(&self.name).neq(lit(*invalid)));
                }
                value
                    .clone()
                    .lazy()
                    .select([expression])
                    .collect()
                    .unwrap()
                    .column("literal")
                    .unwrap()
                    .bool()
                    .unwrap()
                    .to_owned()
            }
            _ => panic!("Column type {:#?} is not Int", dtype),
        }
    }
}

impl Predict for DiscreteRule<f32> {
    fn predict(&self, value: DataFrame) -> ChunkedArray<BooleanType> {
        let dtype = value.column(&self.name).unwrap().dtype();
        match dtype {
            DataType::Float32 => {
                let mut expression = lit(false);
                for valid in &self.valid {
                    expression = expression.or(col(&self.name).eq(lit(*valid)));
                }
                for invalid in &self.invalid {
                    expression = expression.and(col(&self.name).neq(lit(*invalid)));
                }
                value
                    .clone()
                    .lazy()
                    .select([expression])
                    .collect()
                    .unwrap()
                    .column(&self.name)
                    .unwrap()
                    .bool()
                    .unwrap()
                    .to_owned()
            }
            _ => panic!("Column type {:#?} is not Float", dtype),
        }
    }
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_continuous_rule_check() {
        let rule = ContinuousRule {
            name: "a".to_string(),
            rule_type: RuleType::Continuous,
            ranges: vec![Range { min: 0.0, max: 1.0 }, Range { min: 1.0, max: 2.0 }],
        };
        assert_eq!(rule.check(0.5), true);
        assert_eq!(rule.check(1.5), true);
        assert_eq!(rule.check(2.5), false);
    }

    #[test]
    fn test_continuous_rule_predict() {
        let rule = ContinuousRule {
            name: "a".to_string(),
            rule_type: RuleType::Continuous,
            ranges: vec![Range { min: f32::NEG_INFINITY, max: 1.0 }, Range { min: 1.0, max: 2.0 }],
        };

        let s = Series::new("a", [0.5, 1.5, 2.5]);

        let df = DataFrame::new(vec![s]).unwrap();
        let b = rule.predict(df);
        assert!(b
            .into_series()
            .eq(&Series::new("literal", [true, true, false])));
    }

    #[test]
    fn test_discrete_rule_check() {
        let rule = DiscreteRule {
            name: "a".to_string(),
            rule_type: RuleType::Discrete,
            valid: vec![1, 2, 3],
            invalid: vec![4, 5, 6],
            space: vec![7, 8, 9],
        };
        assert_eq!(rule.check(1), true);
        assert_eq!(rule.check(4), false);
        assert_eq!(rule.check(7), false);
    }

    #[test]
    fn test_discrete_rule_predict() {
        let rule = DiscreteRule {
            name: "a".to_string(),
            rule_type: RuleType::Discrete,
            valid: vec![1, 2, 3],
            invalid: vec![4, 5, 6],
            space: vec![1, 2, 3, 4, 5, 6, 7],
        };

        let s = Series::new("a", [1, 4, 7]);
        let df = DataFrame::new(vec![s]).unwrap();
        let b = rule.predict(df);
        assert!(b
            .into_series()
            .eq(&Series::new("literal", [true, false, false])));
    }
}
