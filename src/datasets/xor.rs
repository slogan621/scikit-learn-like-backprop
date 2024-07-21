use polars::frame::DataFrame;
use crate::model::model::Model;

struct Xor;

impl Model for Xor {
    fn load_data() -> DataFrame {
        todo!("implement me");
    }

    fn split_data(split_point: u32) -> (Vec<u8>, Vec<u8>) {
        todo!("implement me");
    }
}