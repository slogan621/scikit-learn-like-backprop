use arff;
use crate::datasets::error::DatasetError;

fn remove_whitespace(s: &mut String) {
    //s.retain(|c| !c.is_whitespace());
    *s = s.replace(&['\t'][..], "");
}

pub fn parse_minst(data: &mut Vec<u8>) -> Result<Vec<Vec<u8>>, DatasetError> {
    /*
    let mut nows = data.to_string(); 
    remove_whitespace(&mut nows);
    let foo = &nows[0..20480];
    println!("after remove ws nows {:?}", foo);
    */
    //let arfres: Vec<u8> = arff::flat_from_str(&data).unwrap();

    // effectively search and replace. arff barffs on tabs
    data.retain_mut(|x| if *x == b'\t' {
        *x = b' ';
        true
    } else {
        true
    });
    //let asstr = String::from_utf8(data.to_vec());
    //let arfres: Vec<Vec<u8>> = arff::from_str(&asstr.unwrap()).unwrap();
    let asstr = String::from_utf8(data.to_vec())?;
    let arfres: Vec<Vec<u8>> = arff::from_str(&asstr)?;
    println!("arfres len is {:?}", arfres.len());
    println!("arfres first entry is {:?}", arfres[0]);
    let len = arfres[0].len();
    println!("length of first entry is {:?}", len);
    println!("first entry classification is {:?}", arfres[0][len - 1]);
    Ok(arfres)
}