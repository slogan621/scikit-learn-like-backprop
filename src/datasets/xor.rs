use polars::frame::DataFrame;
use crate::model::model::Model;

struct Xor;

impl Model for Xor {
    fn load_data() -> DataFrame {
        todo!("implement me");
    }

    fn split_data(df: &DataFrame) -> (DataFrame, DataFrame) {
        todo!("implement me");
    }
}