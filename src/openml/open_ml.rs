
use crate::openml::web_access;
use crate::openml::error::DatasetError;
use crate::openml::minst::parse_minst;
use serde_json::Value;
use polars::prelude::*;

#[derive(Default, Clone, Debug)]
pub enum MLDataType {
    #[default]
    Minst,
}

#[derive(Default, Clone, Debug)]
enum OpenMLParser {
    #[default]
    /// this is the most efficient parser. However, it requires pandas to be installed and can only open dense datasets.
    Pandas,
    /// this is a pure Python ARFF parser that is much less memory- and CPU-efficient. It deals with sparse ARFF datasets.
    Arff,
    /// the parser is chosen automatically such that "liac-arff" is selected for sparse ARFF datasets, otherwise "pandas" is selected.
    Auto,
}

#[derive(Clone, Debug)]
pub enum OpenMLParserRet {
    // Vector of vectors
    VecOfVecs(Vec<Vec<u8>>),
    /// A Pandas (Polars) dataframe
    Pandas(DataFrame),
}

impl Default for OpenMLParserRet {
    fn default() -> Self { OpenMLParserRet::VecOfVecs(vec![])}
}

#[derive(Default, Clone, Debug)]
pub struct FetchOpenMLBuilder  {
    /// String identifier of the dataset. Note that OpenML can have multiple datasets with the same name.
    name: Option<String>,
    /// Version of the dataset. Can only be provided if also name is given. If ‘active’ the oldest version that’s still active is used. Since there may be more than one active version of a dataset, and those versions may fundamentally be different from one another, setting an exact version is highly recommended.
    version: Option<u16>,
    /// OpenML ID of the dataset. The most specific way of retrieving a dataset. If data_id is not given, name (and potential version) are used to obtain a dataset.
    data_id: Option<u16>,
    /// Specify another download and cache folder for the data sets. By default all scikit-learn data is stored in ‘~/scikit_learn_data’ subfolders.
    data_home: Option<String>,
    /// Specify the column name(s) in the data to use as target. If ‘default-target’, the standard target column a stored on the server is used. If None, all columns are returned as data and the target is None. If list (of strings), all columns with these names are returned as multi-target (Note: not all scikit-learn classifiers can handle all types of multi-output combinations).
    target_columns: Vec<String>,
    /// Whether to cache the downloaded datasets into data_home.
    cache: bool,
    /// If True, returns (data, target) instead of a Bunch object. See below for more information about the data and target objects.
    return_x_y: bool,
    /// If True, the data is a pandas DataFrame including columns with appropriate dtypes (numeric, string or categorical). The target is a pandas DataFrame or Series depending on the number of target_columns. The Bunch will contain a frame attribute with the target and the data. If return_X_y is True, then (data, target) will be pandas DataFrames or Series as describe above.
    /// If as_frame is ‘auto’, the data and target will be converted to DataFrame or Series as if as_frame is set to True, unless the dataset is stored in sparse format.
    /// If as_frame is False, the data and target will be NumPy arrays and the data will only contain numerical values when parser="liac-arff" where the categories are provided in the attribute categories of the Bunch instance. When parser="pandas", no ordinal encoding is made.
    /// Changed in version 0.24: The default value of as_frame changed from False to 'auto' in 0.24.
    as_frame: bool,
    ///Number of retries when HTTP errors or network timeouts are encountered. Error with status code 412 won’t be retried as they represent OpenML generic errors.
    n_retries: i16, 
    /// Number of milliseconds between retries.
    delay: u32,
    /// Parser used to load the ARFF file. Two parsers are implemented:
    /// "pandas": this is the most efficient parser. However, it requires pandas to be installed and can only open dense datasets.
    /// "liac-arff": this is a pure Python ARFF parser that is much less memory- and CPU-efficient. It deals with sparse ARFF datasets.
    /// If "auto", the parser is chosen automatically such that "liac-arff" is selected for sparse ARFF datasets, otherwise "pandas" is selected.
    parser: OpenMLParser,
    data_type: MLDataType,
    base_url: String,
}

impl FetchOpenMLBuilder {
    pub fn new() -> Self {
        return FetchOpenMLBuilder {
            n_retries: 3,
            cache: true,
            delay: 1000,
            // example https://www.openml.org/api/v1/json/data/31
            base_url: "https://www.openml.org/api/v1/json/data".to_string(),
            ..Default::default()
        };
    } 

    pub fn with_name(&mut self, _name: String) -> &mut FetchOpenMLBuilder {
        todo!();
    }
    
    pub fn with_version(mut self, version: u16) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_data_id(mut self, data_id: u16) -> Self {
        self.data_id = Some(data_id);
        self
    }

    pub fn with_data_home(mut self, data_home: String) -> Self  {
        self.data_home = Some(data_home);
        self
    }

    pub fn with_target_columns(mut self, columns: Vec<String>) -> Self {
        self.target_columns = columns;
        self
    }

    pub fn with_cache(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    pub fn with_return_x_y(mut self, return_x_y: bool) -> Self {
        self.return_x_y = return_x_y;
        self
    }

    pub fn with_as_frame(mut self, as_frame: bool) -> Self {
        self.as_frame = as_frame;
        self
    }

    pub fn with_n_retries(mut self, n_retries: i16) -> Self {
        self.n_retries = n_retries;
        self
    }

    pub fn with_delay(mut self, delay: u32) -> Self {
        self.delay = delay;
        self
    }

    fn with_parser(mut self, parser: OpenMLParser) -> Self {
        self.parser = parser;
        self
    }

    pub fn with_data_type(mut self, data_type: MLDataType) -> Self {
        self.data_type = data_type;
        self
    }

    pub fn fetch_openml(&self) -> Result<OpenMLParserRet, DatasetError>{
        let ret; 
        match self.data_id {
            Some(val) => {
                let path = format!("{}/{}", self.base_url, val);
                let data = web_access::get(&path, false)?; 
                let v: Value = serde_json::from_slice(&data)?;
                // now get the url of the actual data
                let data_url = &v["data_set_description"]["url"];
                // read the data
                let mut data = web_access::get(&data_url.as_str().unwrap(), self.cache)?; 
                match self.data_type {
                    MLDataType::Minst => {
                        ret = parse_minst(&mut data)?;
                    },
                }
            },
            _ => {
                return Err(DatasetError::MissingID); 
            },
        }
        match self.as_frame { 
            true => {
                let mut pixels : Vec<Series> = vec![];
                let mut digits : Vec<u8> = vec![];
                for val in ret.clone() {
                    let y = &val[0..val.len() - 1];
                    let list = Series::new("pixels", y);
                    pixels.push(list);
                    digits.push(val[val.len() - 1]);
                }

                let df: DataFrame = df!(
                    "pixels" => Series::new("pixels", pixels),
                    "digit" => Series::new("digit", digits),
                )
                .unwrap();

                /* Following shows how to get 3 rows (1, 2, 3) from the df: 
                let indices = IdxCa::new("idx", 1..4);
                let new_df = df.take(&indices).unwrap();
                println!("df 1..4 {:?}", new_df);
                */

                /* Following shows how to get the pixels column from the 
                   dataframe 
                let column_pixels: &Series = new_df.column("pixels").unwrap();
                println!("column_pixels is {:?}", column_pixels);
                */
                return Ok(OpenMLParserRet::Pandas(df));
            },
            _ => {},
        }
        // return default
        Ok(OpenMLParserRet::VecOfVecs(ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_download_mnist_from_openml_as_dataframe() {
        let builder = FetchOpenMLBuilder::new().with_data_id(554).with_data_type(MLDataType::Minst).with_cache(true).with_as_frame(true);

        println!("builder is {:?}", builder.clone());
        let data = builder.fetch_openml().unwrap();
        match data {
            OpenMLParserRet::Pandas(df) => {
                assert_eq!(df.shape(), (70000, 2));
            },
           OpenMLParserRet::VecOfVecs(_) => { panic!("expected pandas but got VecOfVec"); }
        }
    }

    #[test]
    fn can_download_mnist_from_openml_as_vec_of_vec() {
        let builder = FetchOpenMLBuilder::new().with_data_id(554).with_data_type(MLDataType::Minst).with_cache(true);

        let data = builder.fetch_openml().unwrap();
        match data {
            OpenMLParserRet::VecOfVecs(v) => {
                assert_eq!(v.len(), 70000);
                let csized: Vec<_> = v.clone().into_iter().filter(|x| x.len() == 785).collect();
                assert_eq!(v.len(), csized.len());

            },
            OpenMLParserRet::Pandas(_) => { panic!("expected VecOfVec but got Pandas"); }
        }
    }
}
