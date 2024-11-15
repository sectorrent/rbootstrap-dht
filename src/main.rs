use std::process::exit;
use rlibdht::kad::kademlia_base::KademliaBase;
use rlibdht::kademlia::Kademlia;
pub mod daemon;

fn main() {
    if let Err(err) = daemon::daemonize() {
        eprintln!("Daemonization failed: {}", err);
        exit(1);
    }

    let dht = Kademlia::default();
    dht.get_routing_table().lock().unwrap().set_secure_only(false);
    dht.bind(6881);
    dht.join_thread();
}
