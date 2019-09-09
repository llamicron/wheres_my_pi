extern crate reqwest;
extern crate pnet;

use std::{io, fs};
use std::process::exit;
use std::net::IpAddr;
use std::path::PathBuf;
use std::fmt::Write;
use std::collections::HashMap;

use pnet::datalink;

fn get_webhook() -> String {
    let mut path = PathBuf::from(std::env::home_dir().unwrap());
    path.push(".wheres_my_pi");
    match fs::read_to_string(path) {
        Ok(webhook) => return webhook,
        Err(why) => {
            println!("An error occured, could not retrieve webhook: {}", why);
            exit(1);
        }
    }
}

fn get_interfaces() -> Vec<IpAddr> {
    let mut interfaces: Vec<IpAddr> = vec![];
    for iface in datalink::interfaces() {
        if !iface.is_loopback() {
            for ip in iface.ips {
                if ip.is_ipv4() {
                    interfaces.push(ip.ip());
                }
            }
        }
    };
    interfaces
}

fn get_payload(interfaces: &Vec<IpAddr>) -> HashMap<&str, String> {
    let mut payload = HashMap::new();

    let mut message = String::from("Following IP addresses found:\n");
    for ip in interfaces {
        write!(message, " 🦀  {}\n", ip).expect("Couldn't write to message");
    }

    payload.insert("content", message);
    payload
}

fn main() {
    let webhook = get_webhook();
    let interfaces = get_interfaces();
    let payload = get_payload(&interfaces);

    let client = reqwest::Client::new();
    client.post(&webhook)
          .json(&payload)
          .send()
          .expect("Something went wrong, could not send :(");
}

