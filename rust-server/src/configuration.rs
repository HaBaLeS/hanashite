extern crate toml;

use serde::Deserialize;

use std::fs;
use std::path::Path;

static mut CONFIG: Option<Box<Config>> = None;

#[derive(Debug,Deserialize)]
pub struct Runtime {
    #[serde(default = "Runtime::default_threads")]
    pub threads: usize,
    #[serde(default = "Runtime::default_thread_name")]
    pub thread_name: String,
    #[serde(default = "Runtime::default_thread_stack")]
    pub thread_stack: usize,


}

#[derive(Debug,Deserialize)]
pub struct Server {
    #[serde(default = "Server::default_port")]
    pub port: u16,
    #[serde(default = "Server::default_ip")]
    pub ip: String
}

#[derive(Debug,Deserialize)]
pub struct Config {
    #[serde(default = "default_section")]
    pub runtime: Runtime,
    #[serde(default = "default_section")]
    pub server: Server
}

impl Runtime {

    fn default_threads() -> usize {
        4
    }

    fn default_thread_name() -> String {
        "hana-work".to_string()
    }

    fn default_thread_stack() -> usize {
        3 * 1024 * 1024
    }
}

impl Server {

    fn default_port() -> u16 {
        9876
    }

    fn default_ip() -> String {
        "0.0.0.0".to_string()
    }

}

fn default_section<'a, T: Deserialize<'a>>() -> T {
    match toml::from_str("") {
        Ok(val) => val,
        Err(e) => panic!("Default not working: {}", e)
    }
}

impl Default for Server {
    fn default() -> Self {
        match toml::from_str("") {
            Ok(val) => val,
            Err(e) => panic!("Default server not working: {}", e)
        }
    }
}


pub fn init(path: &Path) {
    let config_file = fs::read_to_string(path).expect("Config File not found !");
    match toml::from_str(&config_file) {
        Ok(file) => unsafe { CONFIG = Some(Box::new(file)) },
        Err(e) => panic!("Error in config file: {}", e)
    };
}

pub fn cfg() -> &'static Config {
    unsafe {
        match &CONFIG {
            Some(val) => return &val,
            None => panic!("Access Config before initialization !")
        }
    }
}
#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn testconfig() {
        crate::configuration::init(Path::new("resources/empty.toml"));
        let myconfig = crate::configuration::cfg();
        assert_eq!(myconfig.server.port, 9876);
        assert_eq!(myconfig.server.ip, "0.0.0.0");
        println!("Config: {:#?}", myconfig);
    }

}