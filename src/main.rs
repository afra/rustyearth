extern crate iron;

use std::sync::{Mutex, Arc};
use std::fs::File;

use iron::method::Method;
use iron::middleware::Handler;
use iron::prelude::*;
use iron::headers::{CacheControl, CacheDirective, ContentType};
use iron::status;

pub struct SpaceStatus {
    pub status: bool,
    pub open: String,
    pub close: String,
}

pub struct SpaceApi {
    pub space: String,
    pub location: String,
    pub status: SpaceStatus,
}

impl SpaceApi {
    /* call when authorized by the token */
    fn open(&mut self) {
        self.status.status = true;
    }
    fn close(&mut self) {
        self.status.status = false;
    }
}

impl SpaceIron {
    /* implement all Get requests */
    fn get(&self, request: &mut Request) -> IronResult<Response> {
        match request.url.path()[0] {
            "" => self.index(request),
            "status" => {
                let status = self.space.lock().unwrap().status.status;
                Ok(Response::with((status::Ok, format!("Status {}", status))))
            },
            "status.png" => {
                let status = &self.space.lock().unwrap().status;
                self.status_png(status)
            },
            _ => Ok(Response::with((status::NotFound, format!("Not found {}", request.url))))
        }
    }

    fn index(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Index")))
    }

    fn status_png(&self, status: &SpaceStatus) -> IronResult<Response> {
        let file = match status.status {
            true => &status.open,
            false => &status.close,
        };
        
        let mut resp = Response::with((status::Ok, File::open(file).unwrap()));
        resp.headers.set(CacheControl(vec![CacheDirective::MaxAge(86400u32)]));
        resp.headers.set(ContentType("image/png".parse().unwrap()));
        Ok(resp)
    }
}

impl SpaceIron {
    /* implement all Put requests */
    fn put(&self, request: &mut Request) -> IronResult<Response> {
        match request.url.path()[0] {
            "status" => self.write_status(request),
            _ => Ok(Response::with((status::NotFound, format!("Not found {}", request.url.path()[0])))),
        }
    }

    fn write_status(&self, request: &mut Request) -> IronResult<Response> {
        /* Check token */
        if request.url.path()[1] != self.token {
            Ok(Response::with((status::Forbidden, "Wrong Token")))
        } else {
            match request.url.path()[2] {
                "0" => {
                    self.space.lock().unwrap().close();
                    Ok(Response::with((status::Ok, "closed")))
                },
                "1" => {
                    self.space.lock().unwrap().open();
                    Ok(Response::with((status::Ok, "open")))
                },
                _ => Ok(Response::with((status::NotFound, "Not Found"))),
            }
        }
    }
}

struct SpaceIron {
    pub space: Box<Arc<Mutex<SpaceApi>>>,
    pub token: String,
}

impl Handler for SpaceIron {
    fn handle(&self, request: & mut Request) -> IronResult<Response> {
        match request.method {
            Method::Get => self.get(request),
            Method::Put => self.put(request),
            _ => Ok(Response::with((status::NotFound, "Not found"))),
        }
    }
}

fn main() {
    let space = SpaceApi {
        space: String::from("Abteilung für Redundanz Abteilung"),
        location: String::from("Mar 30, 10317 Berlin"),
        status: SpaceStatus {
            status: false,
            open: String::from("assets/open.png"),
            close: String::from("assets/close.png"),
        },
    };
    let space_mutexed = std::sync::Arc::new(std::sync::Mutex::new(space));
    let space_boxed = Box::new(space_mutexed.clone());

    let spaceiron = SpaceIron {
        space: space_boxed,
        token: String::from("Hee2noh8aic3iech"),
    };

    Iron::new(spaceiron).http("localhost:3000").unwrap();
}
