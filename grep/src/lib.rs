#![allow(unused)]
use failure::{err_msg, Error};

use std::fs::File;
use std::io::{BufReader, Read};

use std::collections::HashSet;

/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug)]
pub struct Flags {
    print_line_number: bool,    // -n
    print_only_filenames: bool, // -l
    ignore_case: bool,          // -i
    invert_search: bool,        // -v
    match_entire_line: bool,    // -x
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut print_line_number = false;
        let mut print_only_filenames = false;
        let mut ignore_case = false;
        let mut invert_search = false;
        let mut match_entire_line = false;

        for &flag in flags {
            match flag {
                "-n" => print_line_number = true,
                "-l" => print_only_filenames = true,
                "-i" => ignore_case = true,
                "-v" => invert_search = true,
                "-x" => match_entire_line = true,
                _ => continue,
            };
        }

        Flags {
            print_line_number,
            print_only_filenames,
            ignore_case,
            invert_search,
            match_entire_line,
        }
    }
}

pub fn grep(origin_pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    let files_count = files.len();

    // in case we are case-insensitive we normalize
    // the original pattern to lowercase letters
    let pattern = match flags.ignore_case {
        true => origin_pattern.to_lowercase(),
        false => origin_pattern.to_string(),
    };

    for path in files {
        let file = File::open(path)?;

        grep_file(&mut results, file, &pattern, flags, path, files_count)?;
    }

    Ok(results)
}

fn grep_file(
    results: &mut Vec<String>,
    mut file: File,
    pattern: &str,
    flags: &Flags,
    file_name: &str,
    files_count: usize,
) -> Result<(), Error> {
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    for (line_idx, origin_line) in content.lines().enumerate() {
        let line: String = if flags.ignore_case {
            origin_line.to_lowercase()
        } else {
            origin_line.to_string()
        };

        let line_matched = match flags.match_entire_line {
            true => line == pattern,
            false => line.contains(pattern),
        };

        let line_satisified = match flags.invert_search {
            true => !(line_matched),
            false => line_matched,
        };

        if line_satisified {
            if flags.print_only_filenames {
                results.push(file_name.to_string());
                return Ok(());
            } else {
                let mut result = Vec::<String>::new();

                if files_count > 1 {
                    result.push(format!("{}:", file_name));
                }

                if flags.print_line_number {
                    result.push(format!("{}:", line_idx + 1));
                }

                result.push(origin_line.to_string());

                results.push(result.join(""));
            }
        }
    }

    Ok(())
}
