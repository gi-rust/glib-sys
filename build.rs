extern crate pkg_config;

use pkg_config::Config;

fn main() {
    Config::new().atleast_version("2.46").find("glib-2.0").unwrap();
}
