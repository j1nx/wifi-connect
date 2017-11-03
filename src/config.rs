use clap::{Arg, App};

use std::env;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::path::PathBuf;
use std::ffi::OsStr;

const DEFAULT_GATEWAY: &str = "192.168.42.1";
const DEFAULT_DHCP_RANGE: &str = "192.168.42.2,192.168.42.254";
const DEFAULT_SSID: &str = "WiFi Connect";
const DEFAULT_TIMEOUT_MS: &str = "15000";
const DEFAULT_UI_PATH: &str = "public";

pub struct Config {
    pub interface: Option<String>,
    pub ssid: String,
    pub password: Option<String>,
    pub clear: bool,
    pub gateway: Ipv4Addr,
    pub dhcp_range: String,
    pub timeout: u64,
    pub ui_path: PathBuf,
}

pub fn get_config() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .value_name("INTERFACE")
                .help("Hotspot interface")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ssid")
                .short("s")
                .long("ssid")
                .value_name("SSID")
                .help("Hotspot SSID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .short("p")
                .long("password")
                .value_name("PASSWORD")
                .help("Hotspot password ")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .value_name("TIMEOUT")
                .help("Connect timeout (milliseconds)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("clear")
                .short("c")
                .long("clear")
                .value_name("CLEAR")
                .help("Clear saved Wi-Fi credentials (default: true)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ui-path")
                .short("u")
                .long("ui-path")
                .value_name("UI_PATH")
                .help("Web UI location")
                .takes_value(true),
        )
        .get_matches();

    let interface: Option<String> = matches.value_of("interface").map_or_else(
        || {
            env::var("PORTAL_INTERFACE").ok()
        },
        |v| Some(v.to_string()),
    );

    let ssid: String = matches.value_of("ssid").map_or_else(
        || {
            env::var("PORTAL_SSID").unwrap_or_else(|_| DEFAULT_SSID.to_string())
        },
        String::from,
    );

    let password: Option<String> = matches.value_of("password").map_or_else(
        || {
            env::var("PORTAL_PASSPHRASE").ok()
        },
        |v| Some(v.to_string()),
    );

    let clear = matches.value_of("clear").map_or(true, |v| !(v == "false"));

    let gateway = Ipv4Addr::from_str(&matches.value_of("gateway").map_or_else(
        || {

            env::var("PORTAL_GATEWAY").unwrap_or_else(|_| DEFAULT_GATEWAY.to_string())
        },
        String::from,
    )).expect("Cannot parse gateway address");

    let dhcp_range = matches.value_of("dhcp-range").map_or_else(
        || {
            env::var("PORTAL_DHCP_RANGE").unwrap_or_else(|_| DEFAULT_DHCP_RANGE.to_string())
        },
        String::from,
    );

    // TODO: network_manager receives the timeout in seconds, should be ms instead.
    let timeout = u64::from_str(&matches.value_of("timeout").map_or_else(
        || {

            env::var("CONNECT_TIMEOUT").unwrap_or_else(|_| DEFAULT_TIMEOUT_MS.to_string())
        },
        String::from,
    )).expect("Cannot parse connect timeout") / 1000;

    let ui_path = get_ui_path(matches.value_of("ui-path"));

    Config {
        interface: interface,
        ssid: ssid,
        password: password,
        clear: clear,
        gateway: gateway,
        dhcp_range: dhcp_range,
        timeout: timeout,
        ui_path: ui_path,
    }
}

fn get_ui_path(cmd_ui_path: Option<&str>) -> PathBuf {
    if let Some(ui_path) = cmd_ui_path {
        return PathBuf::from(ui_path);
    }
    
    if let Ok(ui_path) = env::var("UI_PATH") {
        return PathBuf::from(ui_path);
    }

    if let Some(install_ui_path) = get_install_ui_path() {
        return install_ui_path;
    }
    
    PathBuf::from(DEFAULT_UI_PATH)
}


/// Checks whether `WiFi Connect` is running from install path and whether the
/// UI directory is present in a corresponding location
/// e.g. /usr/local/sbin/wifi-connect -> /usr/local/share/wifi-connect/ui
fn get_install_ui_path() -> Option<PathBuf> {
    if let Ok(exe_path) = env::current_exe() {
        if let Ok(mut path) = exe_path.canonicalize() {
            path.pop();

            match path.file_name() {
                Some(file_name) => {
                    if file_name != OsStr::new("sbin") {
                        // not executing from `sbin` folder 
                        return None;
                    }
                },
                None => return None
            }

            path.pop();
            path.push("share");
            path.push(env!("CARGO_PKG_NAME"));
            path.push("ui");

            if path.is_dir() {
                return Some(path);
            }
        }
    }

    None
}
