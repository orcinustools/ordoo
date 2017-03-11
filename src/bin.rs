extern crate ordoo;
extern crate pencil;
extern crate rustc_serialize;

use ordoo::sock::Docker;
use pencil::{Pencil, Request, PencilResult, jsonify};

fn main() {
    let mut app = Pencil::new("/ordoo");
    app.get("/", "ping", ping);
    app.get("/info", "info", info);
    app.get("/images", "images", images);
    app.get("/nodes", "nodes", nodes);
    app.get("/tasks", "tasks", tasks);
    app.get("/services", "services", services);
    app.get("/volumes", "volumes", volumes);
    app.get("/networks", "networks", networks);
    app.get("/swarm", "swarm", swarm);
    app.run("localhost:5000");
}

fn ping(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.ping())
}

fn info(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.info())
}

fn images(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.images())
}

fn nodes(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.nodes())
}

fn tasks(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.tasks())
}

fn services(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.services())
}

fn volumes(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.volumes())
}

fn networks(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.networks())
}

fn swarm(_: &mut Request) -> PencilResult {
    let docker = Docker::new();
    jsonify(&docker.swarm())
}