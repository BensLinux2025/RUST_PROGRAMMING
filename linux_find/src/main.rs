use std::env;
use walkdir::WalkDir;
use regex::Regex;
use std::ffi::OsStr;

fn find_files_by_pattern(root: &str, pattern: &str) {
    let re = Regex::new(pattern).unwrap();

    // Walk through the directories starting at `root`
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // If the entry is a file and matches the pattern
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            if re.is_match(&file_name) {
                println!("Found file: {:?}", path);
            }
        }
    }
}

fn find_files_by_extension(root: &str, extension: &str) {
    // Walk through the directories starting at `root`
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // If the entry is a file and has the given extension
        if path.is_file() && path.extension() == Some(OsStr::new(extension)) {
            println!("Found file: {:?}", path);
        }
    }
}

fn main() {
    // Get command-line arguments: pattern or extension search
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <directory> <pattern|extension>", args[0]);
        return;
    }

    let root = &args[1];
    let query = &args[2];

    // Determine if the query is an extension or a pattern (regex)
    if query.starts_with('.') {
        // Search by extension
        find_files_by_extension(root, query.trim_start_matches('.'));
    } else {
        // Search by pattern (regex)
        find_files_by_pattern(root, query);
    }
}
