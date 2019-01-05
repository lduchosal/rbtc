use crate::config::Config;

use dns_lookup::{lookup_host};

use std::net::IpAddr;
use std::collections::HashSet;

pub struct Resolver {
    dns_seeds: Vec<String>,    
}

impl Resolver {

    pub fn new(dns_seeds: Vec<String>) -> Resolver {
        Resolver { 
            dns_seeds: dns_seeds,
        }
    }

    pub fn ips(&self) -> HashSet<IpAddr> {

        let mut result : HashSet<IpAddr> = HashSet::new();
        let dns_seeds = &self.dns_seeds;
        for seed in dns_seeds {
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