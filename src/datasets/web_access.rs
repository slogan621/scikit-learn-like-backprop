//! Access the OpenML REST API

use std::fs::{File, OpenOptions};
use std::fs;
use std::io::{self, Read, Write};
use curl::easy::Easy;
//use serde_json::{Result, Value};
//use serde_json::{Value};

//use app_dirs::{app_root, AppDataType, AppInfo};
//use futures::{Future, Stream};
//use hyper::Client;
//use hyper_tls::HttpsConnector;
//use tokio_core::reactor::Core;
use directories::{ProjectDirs};

use crate::datasets::error::DatasetError;

//const APP_INFO: AppInfo = AppInfo{name: "openml-rust", author: "openml-rust"};

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
    //       the writer has created but not yet locked the file?

    let dirs = 
    ProjectDirs::from("com", "Foo Corp",  "Bar App").expect("Failed to get cache dir");
    let dirpath = dirs.config_dir().to_path_buf();
    let mut path = dirpath.clone();

    println!("path is {:?}", path);
    path.push(url_to_file(url));
    println!("path with url is {:?}", path);

    loop {
        match File::open(&path) {
            Ok(mut f) => {
                println!("Loading cached {}", url);
                let mut data = Vec::<u8>::new();
                f.read_to_end(&mut data);
                /* 
                let mut data = String::new();
                f.read_to_string(&mut data)?;
                */
                return Ok(data);
            }
            Err(_) => {println!("open failed");}
        }

        println!("creating new file");
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
                println!("Downloading {}", url);
                let data = download(url)?;
                //f.write_all(data.as_bytes())?;
                f.write_all(&data)?;
                return Ok(data);
            }
        }
    }
}

fn remove_whitespace(s: &mut String) {
    //let s = s.replace(&['\t', ')', ',', '\"', '.', ';', ':', '\''][..], "");
    *s = s.replace(&['\t'][..], "");
    //s.retain(|c| !c.is_whitespace());
}

/// Query a URL.
fn download(url: &str) -> Result<Vec<u8>, DatasetError> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    //handle.url("https://www.rust-lang.org/").unwrap();
    println!("download url is '{:?}'", url);
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    //println!("in download data is {:?}", data);
    //let v: Value = serde_json::from_slice(data.as_slice())?;
    //println!("json is {:?}", v);
/* 
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let req = client.get(url.parse()?);

    let mut bytes = Vec::new();
    {
        let work = req.and_then(|res| {
            res.body().for_each(|chunk| {
                bytes.extend_from_slice(&chunk);
                Ok(())
            })
        });
        core.run(work)?
    }
    */
    /* 
    let result = String::from_utf8(data)?;
    Ok(result)
    */
    Ok(data)
}

/// Convert URL to file name for chching
fn url_to_file(s: &str) -> String {
    s.replace('/', "_").replace(':', "")
}
