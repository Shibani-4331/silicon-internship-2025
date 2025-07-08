use walkdir::WalkDir;
use std::fs::{self,File};
use std::io::{BufReader, Read, Write};
use std::io;
use sha2::{Sha256, Digest};
use blake3;
use twox_hash::XxHash64;
use std::hash::Hasher;
use std::path::{Path,PathBuf};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;


#[derive(Serialize)]
struct Duplicategroup{
    hash: String,
    files: Vec<String>,
} 


fn hash_sha256(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;//it tries to open a file to  read
    let mut hasher = Sha256::new();//creates a new SHA256 hasher to feed data
    let mut reader = BufReader::new(file);//creates a buffered reader for efficient reading
    let mut buffer = [0; 1024];//creates a buffer for reading file data like 1KB at a time
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }//read the file continuously and update the hasher
        hasher.update(&buffer[..bytes_read]);
    }
    Some(format!("{:x}", hasher.finalize()))
}

fn hash_blake3(path:&Path)->Option<String> {
    let file = File::open(path).ok()?;
    let mut hasher = blake3::Hasher::new();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Some(hasher.finalize().to_hex().to_string())
}

fn hash_xxhash64(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut hasher = XxHash64::with_seed(0);
    let mut buffer = [0; 1024];
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.write(&buffer[..n]);
    }
    Some(format!("{:x}", hasher.finish()))
}


fn find_folder_recursive

fn main() {
    println!("Enter folder name: ");
    let mut folder_name = String::new();
    io::stdin().read_line(&mut folder_name).unwrap();
    let folder_name = folder_name.trim();
    let folder_path = Path::new(".").join(folder_name);

    if !folder_path.exists() || !folder_path.is_dir() {
        println!("Folder '{}' not found.", folder_name);
        return;
    }

    // Set filters
    let min_size: u64 = 1024; // 1 KB
    let max_size: u64 = 10 * 1024 * 1024; // 10 MB
    let allowed_extensions = vec!["txt", "rs", "jpg", "png", "mp4", "zip"];

    // Walk through folder recursively with filtering
    let file_paths: Vec<PathBuf> = WalkDir::new(&folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            if !path.is_file() {
                return false;
            }
            let metadata = match fs::metadata(path) {
                    Ok(m) => m,
                    Err(_) => return false,
                };
                let size = metadata.len();
                if size < min_size || size > max_size {
                    return false;
                }

            let ext = match path.extension().and_then(|e| e.to_str()) {
                Some(ext) => ext.to_lowercase(),
                None => return false,
            };
            allowed_extensions.contains(&ext.as_str())
        })
        .map(|e| e.path().to_path_buf())
        .collect();

        // Hash and group files
    let hashes: Vec<_> = file_paths
        .par_iter()
        .filter_map(|path| {
            let ext = path.extension()?.to_string_lossy().to_lowercase();
            let hash = match ext.as_str() {
                "txt" | "rs" => hash_sha256(path),
                "jpg" | "jpeg" | "png" | "mp4" => hash_blake3(path),
                "exe" | "zip" => hash_xxhash64(path),
                _ => hash_sha256(path), // default
            }?;
            Some((hash, path.display().to_string()))
        })
        .collect();
        
         let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (hash, path) in hashes {
        map.entry(hash).or_default().push(path);
    }

    let duplicates: Vec<Duplicategroup> = map
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(hash, files)| Duplicategroup { hash, files })
        .collect();

    // Output JSON report
    if duplicates.is_empty() {
        println!(" No duplicate files found.");
    } else {
        let json = serde_json::to_string_pretty(&duplicates).unwrap();
        fs::write("duplicate_report.json", json).unwrap();
        println!(" Duplicate report saved to 'duplicate_report.json'");


         // Ask for delete confirmation
        print!(" Do you want to delete duplicate copies (keep 1 each)? (y/n): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        if answer.trim().eq_ignore_ascii_case("y") {
            let mut deleted_count = 0;
            for group in &duplicates {
                for dup in &group.files[1..] {
                    if let Err(e) = fs::remove_file(dup) {
                        println!(" Could not delete {}: {}", dup, e);
                    } else {
                        println!("🗑 Deleted: {}", dup);
                        deleted_count += 1;
                    }
                }
            }
            println!(" Deleted {} duplicate files.", deleted_count);
        } else {
            println!(" No files were deleted.");
        }
    }

}
