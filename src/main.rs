use konst::{
    iter, string
};
use unicode_segmentation::UnicodeSegmentation;
use std::io;
use std::process;
use std::env;

fn main() {
    const CSV_DATA: &'static str = include_str!("emojis.csv");

    static EMOJI_LIST: [&str; 1540] = iter::collect_const!(&str =>
        string::split(CSV_DATA, "\n"),
        map(string::trim)
    );

    let search_str = get_search_str();

    let mut found_count = 0;
    let mut skipped_count = 0;
    if search_str.len() > 0 {
        for line in EMOJI_LIST {
            if line.to_lowercase().contains(&search_str) {
                if found_count > 20 {
                    skipped_count += 1;
                    continue;
                }

                let fmt_line = clear_line(line);
                println!("{}", fmt_line);
                found_count += 1;
            }
        }
    }

    if skipped_count > 0 {
        println!("...and {} more.", skipped_count);
    }
    
    if found_count == 0 {
        println!("Nothing was found.");
    }
}


fn get_search_str() -> String {
    if env::args().len() > 1 {
        return env::args().skip(1).next().unwrap().trim().to_string();
    }

    println!("This is emoji finder v0.1.0. Type something to find relevant emojis:");

    let mut search_str = String::new();
    let read_result = io::stdin().read_line(&mut search_str);
    if read_result.is_err() {
        println!("Incorrect input");
        process::exit(0);
    }

    search_str = search_str.trim().to_string();
    search_str
}


fn clear_line(line: &str) -> String {
    let mut chars = line.graphemes(true);
    let emoji = chars.next().unwrap();
    for _ in 0..3 {
        chars.next();
    }
    chars.next_back();
    format!("{} {}", emoji, chars.as_str())
}
