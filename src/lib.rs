#![crate_type = "lib"]
#![crate_name = "ordoo"]

extern crate hyper;
extern crate hyperlocal;

pub mod sock;

#[test]
fn ping() {
    let docker = sock::Docker::new();
    docker.ping();
}

#[test]
fn info() {
    let docker = sock::Docker::new();
    docker.info();
}

#[test]
fn images() {
    let docker = sock::Docker::new();
    docker.images();
}

#[test]
fn containers() {
    let docker = sock::Docker::new();
    docker.containers();
}

#[test]
fn nodes() {
    let docker = sock::Docker::new();
    docker.nodes();
}

#[test]
fn tasks() {
    let docker = sock::Docker::new();
    docker.tasks();
}

#[test]
fn services() {
    let docker = sock::Docker::new();
    docker.services();
}

#[test]
fn volumes() {
    let docker = sock::Docker::new();
    docker.volumes();
}

#[test]
fn networks() {
    let docker = sock::Docker::new();
    docker.networks();
}

#[test]
fn swarm() {
    let docker = sock::Docker::new();
    docker.swarm();
}