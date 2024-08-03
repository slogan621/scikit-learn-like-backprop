use polars::frame::DataFrame;
use crate::model::model::Model;
use crate::model::error::ModelError;

struct Petals;

impl Model for Petals {
    fn load_data() -> Result<DataFrame, ModelError> {
        todo!("implement me");
    }
}