use polars::error::PolarsError;

/* 
#[derive(Debug)]
pub enum ModelError {
    UnableToLoadData,
    InvalidTestTrainSplitArgs,
    UnableToConvertDataFrameToVec(PolarsError),
}
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("unable to load data")]
    UnableToLoadData,
    #[error("invalid test_train_split argumenmts")]
    InvalidTestTrainSplitArgs,
    #[error("unable to convert data frame to vec: {0}")]
    UnableToConvertDataFrameToVec(PolarsError),
    #[error("unable to extract field from data at index: {0}")]
    UnableToExtractFieldFromDataFrame(usize)
}
