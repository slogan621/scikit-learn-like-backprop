//! Access the OpenML REST API

use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{self, Read, Write};
use curl::easy::Easy;
use directories::ProjectDirs;

use crate::datasets::error::DatasetError;

pub fn get(url: &str, cached: bool) -> Result<Vec<u8>, DatasetError> {
    match cached {
        true => {
            get_cached(url)
        },
        false => {
            download(url)
        }
    }
}

/// Query a URL. If possible read the response from local cache
fn get_cached(url: &str) -> Result<Vec<u8>, DatasetError> {
    // todo: is there a potential race condition with a process locking the file for reading while
    // the writer has created but not yet locked the file?

    let dirs = 
    ProjectDirs::from("com", "Foo Corp",  "Bar App").expect("Failed to get cache dir");
    let dirpath = dirs.config_dir().to_path_buf();
    let mut path = dirpath.clone();

    path.push(url_to_file(url));

    loop {
        match File::open(&path) {
            Ok(mut f) => {
                let mut data = Vec::<u8>::new();
                let _ = f.read_to_end(&mut data);
                return Ok(data);
            }
            Err(_) => {println!("open failed");}
        }

        match fs::create_dir(dirpath.clone()) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
        match OpenOptions::new().create_new(true).write(true).open(&path) {
            Err(e) => {
                // todo: is this the correct io error raised if another thread has locked the file currently?
                if let io::ErrorKind::PermissionDenied = e.kind() {
                    continue;
                }
                println!("Error while opening cache for writing: {:?}", e);
                return Err(e.into());
            }
            Ok(mut f) => {
                let data = download(url)?;
                f.write_all(&data)?;
                return Ok(data);
            }
        }
    }
}

fn remove_whitespace(s: &mut String) {
    *s = s.replace(&['\t'][..], "");
}

/// Query a URL.
fn download(url: &str) -> Result<Vec<u8>, DatasetError> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    Ok(data)
}

/// Convert URL to file name for chching
fn url_to_file(s: &str) -> String {
    s.replace('/', "_").replace(':', "")
}
