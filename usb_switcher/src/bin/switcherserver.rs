use likemod::{ModLoader, ModParamValue, ModParams, ModUnloader};
use serde_derive::Deserialize;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::{fs, io, thread, time};
use toml;
use uname;

#[derive(Deserialize)]
struct Config {
    port: u16,
    mounts: Vec<String>,
}

fn main() -> io::Result<()> {
    let config = fs::read_to_string("usbswitcher.toml")?;
    let config: Config = toml::de::from_str(&config).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7424")?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        for byte in stream.bytes() {
            let byte = byte.unwrap();
            // Maybe doing this via sysfs is nicer than this whole spiel with unloading & loading
            let unloader = ModUnloader::new();
            unloader.unload_sync("g_mass_storage", true).unwrap();
            let delay = time::Duration::from_secs(1);
            thread::sleep(delay);
            if let Some(mount) = config.mounts.get(byte as usize) {
                println!("{}", mount);
                let mut params = ModParams::new();
                params.insert("file".to_string(), ModParamValue::Str(mount.to_string()));
                let mut loader = ModLoader::default();
                // Let's emulate where modprobe searches
                let release = uname::uname().unwrap().release;
                let file = format!("/lib/module/{}/g_mass_storage", release);
                let file = fs::File::open(file).unwrap();
                loader
                    .set_parameters(params)
                    .load_module_file(&file)
                    .unwrap();
            }
        }
    }
    Ok(())
}
