use std::fs;

pub fn clean_temp_and_prefetch() -> (u32, u64) {
    let temp_dir = std::env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());
    let prefetch_dir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string()) + "\\Prefetch";

    let (t_count, t_size) = rm_dir_content(&temp_dir);
    let (p_count, p_size) = rm_dir_content(&prefetch_dir);

    (t_count + p_count, t_size + p_size)
}

fn rm_dir_content(dir: &str) -> (u32, u64) {
    let mut deleted_count = 0;
    let mut freed_bytes = 0;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            
            if path.is_file() {
                if fs::remove_file(&path).is_ok() {
                    deleted_count += 1;
                    freed_bytes += size;
                }
            } else if path.is_dir() {
                if fs::remove_dir_all(&path).is_ok() {
                    deleted_count += 1;
                    freed_bytes += size;
                }
            }
        }
    }

    (deleted_count, freed_bytes)
}
