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

pub trait Model {
    fn load_data() -> DataFrame;
    fn test_train_split(df: &DataFrame, test_size: Option<f32>, train_size: Option<f32>) -> (DataFrame, DataFrame, DataFrame, DataFrame);
}