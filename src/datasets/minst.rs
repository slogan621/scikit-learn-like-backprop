use polars::frame::DataFrame;
use polars::datatypes::IdxCa;
use polars::prelude::NamedFrom;
use crate::model::model::Model;
use crate::openml::open_ml::{OpenMLParserRet, FetchOpenMLBuilder, MLDataType};

struct Minst;

impl Model for Minst {
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

    fn test_train_split(df: &DataFrame, test_size: Option<f32>, train_size: Option<f32>) -> (DataFrame, DataFrame, DataFrame, DataFrame) {
        todo!("implement me");
        /* 
        let foo = df.to_nd
        // select setosa and versicolor
        y = df.iloc[0:100, 4].values
        y = np.where(y == 'Iris-setosa', 0, 1)
        // extract sepal length and petal length
        y = df.iloc[0:100, [0, 2]].values
/*
        let (width, _) = df.shape();
        let w: u32 = width.try_into().unwrap();
        let indices = IdxCa::new("idx", w - 1..w);
        let y = df.take(&indices).unwrap();
        //y = np.where(y == 'Iris-setosa', 0, 1)
        // extract sepal length and petal length
        // pandas X = df.iloc[0:, [0, 1]].values
        let indices = IdxCa::new("idx", 0..w - 1);
        let x = df.take(&indices).unwrap();
        */
        return (x, y)
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_minst_data() {
        let df = Minst::load_data();
        assert_eq!(df.shape(), (70000, 2));
    }

    #[test]
    fn can_load_and_split_minst_data() {
        let df = Minst::load_data();
        assert_eq!(df.shape(), (70000, 2));
        let (x_train, y_train, x_test, y_test) = Minst::test_train_split(&df, None, None);
        /* 
        assert_eq!(x.n_chunks(), 1);
        println!("x is {:?}", x);
        assert_eq!(y.n_chunks(), 1);
        println!("y is {:?}", y);
        */
        assert_eq!(1, 2);
    }
}

