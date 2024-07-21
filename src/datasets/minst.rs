use arff;
use crate::datasets::error::DatasetError;

fn remove_whitespace(s: &mut String) {
    //s.retain(|c| !c.is_whitespace());
    *s = s.replace(&['\t'][..], "");
}

pub fn parse_minst(data: &mut Vec<u8>) -> Result<Vec<Vec<u8>>, DatasetError> {

    // effectively search and replace tabs. upstream has a fix but
    // never released to crates.io (as of today) so this pass is
    // unfortunately required.
    data.retain_mut(|x| if *x == b'\t' {
        *x = b' ';
        true
    } else {
        true
    });
    let asstr = String::from_utf8(data.to_vec())?;
    let arfres: Vec<Vec<u8>> = arff::from_str(&asstr)?;
    Ok(arfres)
}