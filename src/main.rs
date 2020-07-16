use std::env;
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut file_max = String::new();
    let mut line_count_max = 0;
    for path in env::args() {
        visit_dirs(Path::new(&path), &mut |entry| {
            get_max_line_count(entry, &mut file_max, &mut line_count_max)
        })?;
    }
    println!(
        "\"{}\" has the longest function, with {} lines.",
        file_max, line_count_max
    );
    Ok(())
}

fn visit_dirs<F>(dir: &Path, cb: &mut F) -> io::Result<()>
where
    F: FnMut(&DirEntry) -> io::Result<()>,
{
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn get_max_line_count(
    entry: &DirEntry,
    file_max: &mut String,
    line_count_max: &mut u32,
) -> io::Result<()> {
    let file = File::open(entry.path())?;
    let mut line_count = 0;
    let mut line_stack = Vec::new();
    for byte in BufReader::new(file).bytes() {
        let byte = byte?;
        if byte == '\n' as u8 {
            line_count += 1;
        } else if byte == '{' as u8 {
            line_stack.push(line_count);
        } else if byte == '}' as u8 {
            if let Some(last_line_count) = line_stack.pop() {
                let current_line_count = line_count - last_line_count;
                if current_line_count > *line_count_max {
                    *file_max = entry.path().to_string_lossy().to_string();
                    *line_count_max = current_line_count;
                }
            }
        }
    }
    Ok(())
}
