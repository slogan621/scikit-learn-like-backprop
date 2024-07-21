use polars::frame::DataFrame;
use crate::model::model::Model;
use crate::openml::open_ml::{OpenMLParserRet, FetchOpenMLBuilder, MLDataType};

struct Petals;

impl Model for Petals {
    fn load_data() -> DataFrame {
        let builder = FetchOpenMLBuilder::new().with_data_id(554).with_data_type(MLDataType::Minst).with_cache(true).with_as_frame(true);

        let data = builder.fetch_openml().unwrap();
        match data {
            OpenMLParserRet::Pandas(df) => {
                return df;
            },
            _ => { panic!("unexpected result from fetch_openml {:?}", data); }
        }
    }

    fn split_data(split_point: u32) -> (Vec<u8>, Vec<u8>) {
        todo!("implement me");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_petals_data() {
        let df = Petals::load_data();
        assert_eq!(df.shape(), (70000, 2));
    }
}

