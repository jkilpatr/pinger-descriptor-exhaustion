extern crate env_logger;

use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    info!("Starting pinger!");
    loop {
        let (pinger, results) = match Pinger::new(None, None) {
            Ok((pinger, results)) => (pinger, results),
            Err(e) => panic!("Error creating pinger: {}", e),
        };
        pinger.add_ipaddr("8.8.8.8");
        pinger.add_ipaddr("1.1.1.1");
        pinger.add_ipaddr("7.7.7.7");
        pinger.add_ipaddr("2001:4860:4860::8888");
        pinger.ping_once();

        for _x in 0..4 {
            match results.recv() {
                Ok(result) => match result {
                    Idle { addr } => {
                        error!("Idle Address {}.", addr);
                    }
                    Receive { addr, rtt } => {
                        info!("Receive from Address {} in {:?}.", addr, rtt);
                    }
                },
                Err(_) => panic!("Worker threads disconnected before the solution was found!"),
            }
        }
    }
}
