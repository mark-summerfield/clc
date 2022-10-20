// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use std::path::PathBuf;

pub fn abspath(name: &str) -> PathBuf {
    let filename = PathBuf::from(name);
    if filename.is_absolute() {
        filename
    } else {
        filename.canonicalize().unwrap_or(filename)
    }
}

pub fn elide(s: &str, offset: usize, width: usize) -> String {
    assert!(offset + 5 < width);
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= width {
        s.to_string()
    } else {
        let left: String = chars[..offset].iter().collect();
        #[cfg(windows)]
        let ellipsis = "...";
        #[cfg(unix)]
        let ellipsis = "…";
        let i = chars.len() - (width - (offset + ellipsis.len()));
        let right: String = chars[i..].iter().collect();
        format!("{left}{ellipsis}{right}")
    }
}
