use std::path::Path;

pub struct Config {
    pub dns_seeds: Vec<String>,
    pub sqlite_path: &'static Path,
}
