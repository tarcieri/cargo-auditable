use auditable_serde::RawVersionInfo;
use cargo_lock::Lockfile;
use std::str::FromStr;
use std::convert::TryInto;

fn main() {
    let path = std::env::args().skip(1).next().expect("No file specified");
    let file_contents = std::fs::read_to_string(path).unwrap();
    let version_info = RawVersionInfo::from_str(&file_contents).unwrap();
    let lockfile: Lockfile = version_info.try_into().unwrap();
    let lockfile_toml = lockfile.to_string();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    std::io::Write::write_all(&mut stdout, lockfile_toml.as_bytes()).unwrap();
    
}

// fn main() {
//     let path = std::env::args().skip(1).next().expect("No file specified");
//     let file_contents = std::fs::read_to_string(path).unwrap();
//     let version_info = RawVersionInfo::from_toml(&file_contents).unwrap();
//     let stdout = std::io::stdout();
//     let stdout = stdout.lock();
//     serde_json::to_writer(stdout, &version_info).unwrap();
// }