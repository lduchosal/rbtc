use rbtc::network::networkaddr::NetworkAddr;
use rbtc::network::version::Version;
use rbtc::network::verack::VerAck;
use rbtc::network::getaddr::GetAddr;
use rbtc::network::version::Service;
use rbtc::network::message::Payload;
use rbtc::network::message::{Message, Magic};

use rand::Rng;
use std::net::{IpAddr};

pub struct MessageProvider {}

impl MessageProvider {

    pub fn version() -> Vec<Message> {

        let now = chrono::Local::now();
        let mut rng = rand::thread_rng();
        let nonce: u64 = rng.gen();

        let version = Version {
            version: 70002,
            services: Service::Network,
            timestamp: now.timestamp(),
            receiver: NetworkAddr {
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            sender: NetworkAddr {
                services: Service::Network,
                ip: IpAddr::V4("0.0.0.0".parse().unwrap()),
                port: 0
            },
            nonce: nonce,
            user_agent: "/rbtc:0.17.0.1/".to_string(),
            start_height: 557409,
            relay: false,
        };

        let version = Payload::Version(version);
        vec![
            Message {
                magic: Magic::MainNet,
                payload: version
            }
        ]
    }

    pub fn getaddr() -> Vec<Message> {

        vec![
            Message {
                magic: Magic::MainNet,
                payload: Payload::GetAddr(GetAddr {

                })
            },
        ]
    }

    pub fn verack() -> Vec<Message> {
        vec![
            Message {
                magic: Magic::MainNet,
                payload: Payload::VerAck(VerAck {
                    
                })
            },
        ]
    }
}