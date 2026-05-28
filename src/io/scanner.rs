use std::fs;
use std::path::PathBuf;

pub fn scan(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let entries = fs::read_dir(dir)?;

    let results = entries
        .filter_map(|entry| entry.ok()) 
        .map(|e| e.path())              
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("bsa")
        })
        .collect(); 

    Ok(results)
}

pub fn scan_code(dir: &str) -> Result<Vec<PathBuf>, String> {
    scan(dir).map_err(|e| format!("Coudln't read dir {dir}: {e}"))
}

pub fn load_code(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}