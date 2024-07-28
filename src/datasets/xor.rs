use polars::frame::DataFrame;
use crate::model::model::Model;

struct Xor;

impl Model for Xor {
    fn load_data() -> DataFrame {
        todo!("implement me");
    }
    fn test_train_split(df: &DataFrame, test_size: Option<f32>, train_size: Option<f32>) -> (DataFrame, DataFrame, DataFrame, DataFrame) {
        todo!("implement me");
    }
}