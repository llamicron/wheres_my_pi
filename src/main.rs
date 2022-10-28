extern crate reqwest;
extern crate pnet;
extern crate dirs;

use std::fs;
use std::process::exit;
use std::net::IpAddr;
use std::path::PathBuf;
use std::fmt::Write;
use std::collections::HashMap;
use dirs::home_dir;

use pnet::datalink;

/// Reads a webhook from ~/.wheres_my_pi
/// See discord documentation for creating a webhook
fn get_webhook() -> String {
    let mut path = PathBuf::from(home_dir().unwrap());
    path.push(".wheres_my_pi");
    match fs::read_to_string(path) {
        Ok(webhook) => return webhook,
        Err(why) => {
            println!("An error occured, could not retrieve webhook: {}", why);
            exit(1);
        }
    }
}

/// Gets a Vec<IpAddr> of all IPs that are not loopback and are IPv4
/// Usually only one
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

/// Creates a dictionary like this {"content": "..."} to send to Discord
///
/// The message is a formatted string of all the IP addresses with
/// a little crab emoji because it's rust
fn get_payload(interfaces: &Vec<IpAddr>) -> HashMap<&str, String> {
    let mut payload = HashMap::new();

    let mut message = String::from("Following IP addresses found:\n");
    for ip in interfaces {
        write!(message, " 🦀  {}\n", ip).expect("Couldn't write to message");
    }

    payload.insert("content", message);
    payload
}

/// Construct the payload, then send it
fn main() {
    let webhook = get_webhook();
    let interfaces = get_interfaces();
    let payload = get_payload(&interfaces);

    let client = reqwest::blocking::Client::new();

    println!("Addresses found:");
    for addr in &interfaces {
        println!(" - {}", addr);
    }


    client.post(&webhook)
        .json(&payload)
        .send()
        .expect("Something went wrong, could not send :(");
}

