use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::{fs, print};
use super::PWD;

pub static CMD: &str = "cd";
pub static USAGE: &str = "cd <path>";
pub static DES: &str = "Changes the current working directory";

pub fn main(args: &[&str]) {
    let target = if args.is_empty() {
        // cd with no args goes to root (or could go to home directory)
        "/"
    } else {
        args[0]
    };
    
    // Build the target path
    let target_path = resolve_path(target);
    
    // Verify the directory exists
    match fs::list_dir(&target_path) {
        Ok(_) => {
            let normalized = normalize_path(&target_path);
            unsafe { PWD = normalized.leak(); }
        }
        Err(_) => {
            print!("cd: {}: No such directory\n", target);
        }
    }
}

fn resolve_path(path: &str) -> String {
    if path.starts_with('/') {
        // Absolute path
        path.to_string()
    } else if path == "." {
        // Current directory
        unsafe { PWD.to_string() }
    } else if path == ".." {
        // Parent directory
        unsafe {get_parent_dir(PWD)}
    } else if path.starts_with("..") {
        // Path starting with ..
        let remaining = &path[2..].trim_start_matches('/');
        let parent = unsafe{get_parent_dir(PWD)};
        if remaining.is_empty() {
            parent
        } else {
            format!("{}/{}", parent, remaining)
        }
    } else {
        // Relative path
        if unsafe {PWD} == "/" {
            format!("/{}", path)
        } else {
            format!("{}/{}", unsafe{PWD}, path)
        }
    }
}

fn get_parent_dir(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }
    
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    
    if parts.is_empty() {
        return "/".to_string();
    }
    
    if parts.len() == 1 {
        return "/".to_string();
    }
    
    format!("/{}", parts[..parts.len()-1].join("/"))
}

fn normalize_path(path: &str) -> String {
    if path == "/" {
        return "/".to_string();
    }
    
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut normalized = Vec::new();
    
    for part in parts {
        match part {
            "." => continue,
            ".." => {
                normalized.pop();
            }
            _ => normalized.push(part),
        }
    }
    
    if normalized.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", normalized.join("/"))
    }
}