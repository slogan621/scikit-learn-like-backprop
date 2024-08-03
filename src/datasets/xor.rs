use polars::frame::DataFrame;
use crate::model::model::Model;
use crate::model::error::ModelError;

struct Xor;

impl Model for Xor {
    fn load_data() -> Result<DataFrame, ModelError> {
        todo!("implement me");
    }
}