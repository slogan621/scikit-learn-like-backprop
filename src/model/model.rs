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

fn data_frame_to_option_u8_vec(df: &DataFrame, idx: usize) -> Result<Vec<Option<u8>>, ModelError> {
    let s = df.select_at_idx(idx); 
    let as_vec: Vec<Option<u8>> = s.expect("unable to select from df").u8().expect("unable to iconvert to u8").into_iter().collect();
    return Ok(as_vec);
}

fn data_frame_to_u8_vec(df: &DataFrame, idx: usize) -> Result<Vec<u8>, ModelError> {
    let s = df.select_at_idx(idx); 
    let as_vec: Vec<u8> = s.expect("unable to select from df").u8().expect("Unable to convert to u8").into_no_null_iter().collect();
    Ok(as_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::df;

    #[test]
    fn can_convert_dataframe_to_vec_u8() {
        let df = df! [
            "names" => ["a", "b", "c"],
            "values" => [1u8, 2u8, 3u8],
            "values_nulls" => [Some(1u8), None, Some(3u8)]
        ].unwrap();

        let as_vec = data_frame_to_u8_vec(&df, 1).unwrap();
        assert_eq!(as_vec.len(), 3);
        assert_eq!(as_vec, [1u8, 2u8, 3u8]);
        let as_vec = data_frame_to_option_u8_vec(&df, 2).unwrap();
        assert_eq!(as_vec.len(), 3);
        assert_eq!(as_vec, [Some(1u8), None, Some(3u8)]);
    }
}