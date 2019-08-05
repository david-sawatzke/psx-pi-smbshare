use serde_derive::Deserialize;
use std::io::Read;
use std::net::TcpListener;
use std::process::Command;
use std::{fs, io, thread, time};
use toml;

#[derive(Deserialize)]
struct Config {
    port: u16,
    mounts: Vec<String>,
}

fn main() -> io::Result<()> {
    let config = fs::read_to_string("usbswitcher.toml")?;
    let config: Config = toml::de::from_str(&config).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7424")?;
    let mut unloader = Command::new("rmmod");
    unloader.arg("g_mass_storage");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        for byte in stream.bytes() {
            let byte = byte.unwrap();
            // Special shutdown byte, just cause I needed to put it somewhere to easily
            if byte == 0xFF {
                Command::new("shutdown").output().expect("Couldn't reboot");
                continue;
            }
            // Maybe doing this via sysfs is nicer than this whole spiel with unloading & loading
            unloader.output().expect("Couldn't unload");
            let delay = time::Duration::from_secs(1);
            thread::sleep(delay);
            if let Some(mount) = config.mounts.get(byte as usize) {
                println!("{}", mount);
                Command::new("modprobe")
                    .arg("g_mass_storage")
                    .arg(format!("file={}", mount))
                    .output()
                    .expect("Couldn't load");
            }
        }
    }
    Ok(())
}
