use crate::config::Config;

use dns_lookup::{lookup_host};

use std::net::IpAddr;
use std::collections::HashSet;

pub struct Resolver<'a> {
    config: &'a Config,    
}

impl<'a> Resolver<'a> {

    pub fn new(config: &Config) -> Resolver {
        Resolver { 
            config: config,
        }
    }

    pub fn ips(&self) -> HashSet<IpAddr> {

        let mut result : HashSet<IpAddr> = HashSet::new();
        let seeds = &self.config.dns_seeds;
        for seed in seeds {
            let oips = lookup_host(&seed);

            if let Ok(ips) = oips {
                for ip in ips {
                    result.insert(ip);
                }
            }
        }

        result
    }

}