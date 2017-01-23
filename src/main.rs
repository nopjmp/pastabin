extern crate rand;
extern crate hyper;

use std::fs::{File, OpenOptions, remove_file};
use std::str::FromStr;
use std::path::Path;

mod pasteid;
use pasteid::PasteID;

use std::io;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::*;

const ID_SIZE: usize = 8;
const USAGE: &'static [u8] = b"
pastabin 0.0.1 - Minimal pastebin clone in Rust. Manual post required. CLI recommended.

    USAGE

      POST /

        accepts raw data in the body of the request and responds with a URL of
        a page containing the body's content

      GET /<id>

        retrieves the content for the paste with id `<id>`

    EXAMPLE

        Upload a file:
        
            curl --data-binary @file.txt https://pasta.lol/

        Upload from stdin:

            echo \"Hellow, World\" | curl --data-binary @- https://pasta.lol/

        Delete an existing paste:

            curl -X DELETE https://pasta.lol/<id>
    ";

fn retrieve_paste(id: PasteID) -> Option<File> {
    File::open(&id.filename()).ok()
}

fn create_paste(id: &PasteID) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&id.filename())
}

macro_rules! try_handle {
    ($res:ident, $e:expr) => {{
        match $e.ok() {
            Some(v) => v,
            None => {
                *$res.status_mut() = StatusCode::InternalServerError;
                return;
            },
        }
    }}
}

fn handle(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            match req.uri.clone() {
                AbsolutePath(path) => {
                    if path == "/" {
                        res.headers_mut().set(ContentType::plaintext());
                        res.send(USAGE).unwrap();
                    } else if path == "/favicon.ico" {
                        // todo favicon
                        *res.status_mut() = StatusCode::NotFound;
                    } else if let Ok(id) = PasteID::from_str(path.trim_left_matches("/")) {
                        match retrieve_paste(id) {
                            Some(mut file) => {
                                let metadata = try_handle!(res, file.metadata());
                                res.headers_mut().set(ContentLength(metadata.len()));
                                res.headers_mut().set(ContentType::plaintext());
                                io::copy(&mut file, &mut res.start().unwrap()).unwrap();
                            }
                            None => *res.status_mut() = StatusCode::NotFound,
                        }
                    } else {
                        *res.status_mut() = StatusCode::BadRequest;
                    }
                }
                _ => *res.status_mut() = StatusCode::BadRequest,
            }
        }
        hyper::Post => {
            match req.uri.clone() {
                AbsolutePath(path) => {
                    if path != "/" {
                        *res.status_mut() = StatusCode::BadRequest;
                    } else {
                        let mut tries = 0;
                        let mut id = PasteID::new(ID_SIZE);
                        let mut file = create_paste(&id);
                        while file.is_err() && tries < 3 {
                            id = PasteID::new(ID_SIZE);
                            file = create_paste(&id);
                            tries += 1;
                        }
                        if file.is_err() {
                            *res.status_mut() = StatusCode::InternalServerError;
                        } else {
                            // TODO: handle errors
                            try_handle!(res, io::copy(&mut req, &mut try_handle!(res, file)));
                            *res.status_mut() = StatusCode::Created;
                            res.send(format!("https://pasta.lol/{}\n", id).as_bytes())
                                .unwrap();
                        }
                    }
                }
                _ => *res.status_mut() = StatusCode::BadRequest,
            }
        }
        hyper::Delete => {
            match req.uri.clone() {
                AbsolutePath(path) => {
                    if let Ok(id) = PasteID::from_str(path.trim_left_matches("/")) {
                        let filename = id.filename();
                        let path = Path::new(&filename);
                        if path.exists() {
                            try_handle!(res, remove_file(path));
                        } else {
                            *res.status_mut() = StatusCode::NotFound;
                        }
                    } else {
                        *res.status_mut() = StatusCode::BadRequest;
                    }
                }
                _ => *res.status_mut() = StatusCode::BadRequest,
            }
        }
        _ => *res.status_mut() = StatusCode::MethodNotAllowed,
    }
}

fn main() {
    let mut ipv4 = Server::http("127.0.0.1:8080").unwrap();
    ipv4.keep_alive(None);
    let _guardipv4 = ipv4.handle(handle);
}