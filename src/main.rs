extern crate ordoo;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

use ordoo::Docker;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use rustc_serialize::json;

fn main() {
    let mut router = Router::new();

    let ping = |_: &mut Request| -> IronResult<Response> {
        let docker = Docker::new();
        let ping = docker.ping().unwrap();
        let n = json::encode(&ping).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, n)))
    };

    let info = |_: &mut Request| -> IronResult<Response> {
        let docker = Docker::new();
        let info = docker.info().unwrap();
        let n = json::encode(&info).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, n)))
    };

    let images = |_: &mut Request| -> IronResult<Response> {
        let docker = Docker::new();
        let images = docker.images().list(&Default::default()).unwrap();
        let n = json::encode(&images).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, n)))
    };

    router.get("/", ping, "ping");
    router.get("/info", info, "info");
    router.get("/images", images, "images");
    Iron::new(router).http("localhost:3000").unwrap();
}
