use polars::frame::DataFrame;
use crate::model::model::Model;

struct Petals;

impl Model for Petals {
    fn load_data() -> DataFrame {
        todo!("implement me");
    }

    fn split_data(df: &DataFrame) -> (DataFrame, DataFrame) {
        todo!("implement me");
    }
}