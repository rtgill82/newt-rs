extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("0.52.15")
        .probe("libnewt")
        .unwrap();
}
