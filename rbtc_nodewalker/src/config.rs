use std::path::Path;

pub struct Config {
    pub dns_seeds: Vec<&'static str>,
    pub sqlite_path: &Path,
}
