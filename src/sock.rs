use std::io::Read;
use hyper::Client;
use hyperlocal::{DomainUrl, UnixSocketConnector};

struct Binding {
    endpoint: &'static str,
}

pub struct Docker {
    new: Binding,
}

impl Binding {
    fn new(endpoint: &'static str) -> Binding {
        Binding { endpoint: endpoint }
    }

    fn get(&self, api: &str) -> String {
        let url = self.endpoint;
        docker(url, api)
    }
}

impl Docker {
    pub fn new() -> Docker {
        let binding = Binding::new("/var/run/docker.sock");
        Docker { new: binding }
    }

    pub fn ping(&self) -> String {
        self.new.get("/_ping")
    }

    pub fn info(&self) -> String {
        self.new.get("/info")
    }

    pub fn images(&self) -> String {
        self.new.get("/images/json")
    }

    pub fn containers(&self) -> String {
        self.new.get("/containers/json")
    }

    pub fn nodes(&self) -> String {
        self.new.get("/nodes/json")
    }

    pub fn tasks(&self) -> String {
        self.new.get("/tasks/json")
    }

    pub fn services(&self) -> String {
        self.new.get("/services")
    }

    pub fn volumes(&self) -> String {
        self.new.get("/volumes")
    }

    pub fn networks(&self) -> String {
        self.new.get("/networks")
    }

    pub fn swarm(&self) -> String {
        self.new.get("/swarm")
    }
}

fn docker(url: &str, api: &str) -> String {
    let client = Client::with_connector(UnixSocketConnector);
    let mut response = match client.get(DomainUrl::new(url, api)).send() {
        Ok(response) => response,
        Err(_) => panic!("Error."),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Sorry."),
    }
    return buf;
}
