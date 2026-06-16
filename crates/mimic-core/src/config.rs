use std::{io::Result, path::PathBuf};

fn data_dir() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| {
        eprintln!("[MimicServer] WARNING: dirs::data_dir() returned None, falling back to '.'");
        PathBuf::from(".")
    });
    base.join("MimicServer")
}

fn payloads_dir() -> PathBuf {
    data_dir().join("payloads")
}

pub fn db_path() -> PathBuf {
    data_dir().join("mimic_server_v1.db")
}

pub fn ensure_dirs() -> Result<()> {
    let dir = payloads_dir();
    eprintln!("[MimicServer] Data directory: {:?}", dir.parent().unwrap());
    eprintln!("[MimicServer] DB will be at:  {:?}", db_path());
    std::fs::create_dir_all(&dir)
}
