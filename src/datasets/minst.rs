use polars::frame::DataFrame;
use crate::model::model::Model;
use crate::model::error::ModelError;
use crate::openml::open_ml::{OpenMLParserRet, FetchOpenMLBuilder, MLDataType};

struct Minst;

impl Model for Minst {
    fn load_data() -> Result<DataFrame, ModelError> {
        let builder = FetchOpenMLBuilder::new().with_data_id(554).with_data_type(MLDataType::Minst).with_cache(true).with_as_frame(true);

        let data = builder.fetch_openml().unwrap();
        match data {
            OpenMLParserRet::Pandas(df) => {
                return Ok(df);
            },
            _ => { return Err(ModelError::UnableToLoadData); },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_minst_data() {
        let df = Minst::load_data();
        assert_eq!(df.unwrap().shape(), (70000, 2));
    }

    #[test]
    fn cannnot_load_and_split_minst_data_no_sizes() {
        let df = Minst::load_data().unwrap();
        assert_eq!(df.shape(), (70000, 2));
        let ret = Minst::test_train_split(&df, None, None);

        assert!(ret.is_err());
    }

    #[test]
    fn cannnot_load_and_split_minst_data_both_sizes() {
        let df = Minst::load_data().unwrap();
        assert_eq!(df.shape(), (70000, 2));
        let ret = Minst::test_train_split(&df, Some(0.4), Some(0.6));

        assert!(ret.is_err());
    }

    #[test]
    fn can_load_and_split_minst_data_with_train_size() {
        let df = Minst::load_data().unwrap();
        assert_eq!(df.shape(), (70000, 2));
        let ret = Minst::test_train_split(&df, None, Some(0.6));

        assert!(!ret.is_err());
        let (test, train) = ret.unwrap();
        assert_eq!(test.shape().0, 28_000);
        assert_eq!(train.shape().0, 42_000);
    }

    #[test]
    fn can_load_and_split_minst_data_with_test_size() {
        let df = Minst::load_data().unwrap();
        assert_eq!(df.shape(), (70000, 2));
        let ret = Minst::test_train_split(&df, Some(0.2), None);

        assert!(!ret.is_err());
        let (test, train) = ret.unwrap();
        assert_eq!(test.shape().0, 14_000);
        assert_eq!(train.shape().0, 56_000);
    }
}

