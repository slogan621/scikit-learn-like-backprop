/* TODO: the following could be arg to load_data 
   to direct it to load from OpenML, file, etc.
   But for now, we leave it to the impl to 
   decide where the data comes from.

pub enum LoadMethod {
    OpenML(u32),
    File(FilePath).
    ...
}
*/

use polars::frame::DataFrame;
use crate::model::error::ModelError;

pub trait Model {
    fn load_data() -> Result<DataFrame, ModelError>;
    fn test_train_split(df: &DataFrame, test_size: Option<f32>, train_size: Option<f32>) -> Result<(DataFrame, DataFrame), ModelError> {

        if test_size.is_none() && train_size.is_none() {
            return Err(ModelError::InvalidTestTrainSplitArgs);
        }

        if test_size.is_some() && train_size.is_some() {
            return Err(ModelError::InvalidTestTrainSplitArgs);
        }

        let nrows: f32 = df.shape().0 as f32;
        let size_test = match test_size {
            Some(s) => {
                nrows * s
            },
            _ => {
                // Safety: safe due to earlier is_some checks
                nrows * (1.0 - train_size.unwrap())
            }
        };
        /*
        TODO: maybe add support for shuffle, though in
        polars rust, this can't be done on a DataFrame,
        as in Python version below 
    )
        df = pl.DataFrame({"val": range(100)})

        df = df.sample(fraction=1, shuffle=True)

        See following which has a shuffle arg
        https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sample_n_literal
        */
        let nrows: usize = nrows.ceil() as _;
        let size_test: usize = size_test.ceil() as _;
        let size_train = nrows - size_test;
        assert_eq!(size_test + size_train, nrows);

        let test_res = df.head(Some(size_test));
        let train_res = df.tail(Some(size_train));

        Ok((test_res, train_res))
    }
}