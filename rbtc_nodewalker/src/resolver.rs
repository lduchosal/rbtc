use dns_lookup::{lookup_host};

use std::net::IpAddr;
use std::collections::HashSet;

pub struct Resolver {
    dns_seeds: Vec<String>,    
}

impl Resolver {

    pub fn new(dns_seeds: Vec<String>) -> Resolver {
        trace!("new");

        Resolver { 
            dns_seeds: dns_seeds,
        }
    }

    pub fn ips(&self) -> HashSet<IpAddr> {

        trace!("ips");

        let mut result : HashSet<IpAddr> = HashSet::new();
        let dns_seeds = &self.dns_seeds;
        for seed in dns_seeds {

            info!("ips [seed: {}]", seed);

            let oips = lookup_host(&seed);
            if let Ok(ips) = oips {
                
                info!("ips [ips: {}]", ips.len());

                for ip in ips {
                    result.insert(ip);
                }
            }
        }

        result
    }

}