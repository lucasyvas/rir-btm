#![forbid(unsafe_code)]
#![warn(clippy::all)]

use cfg_if::cfg_if;
use std::fs::read_to_string;
use std::time::Instant;

cfg_if! {
    if #[cfg(not(feature = "parallel"))] {
        use std::collections::HashMap;
    } else {
        use dashmap::DashMap;
        use rayon::prelude::*;
    }
}

fn main() {
    let words = read_to_string("wordlist.txt").unwrap();
    let words = words.trim().split("\n").collect::<Vec<&str>>();

    #[cfg(not(feature = "parallel"))]
    let mut word_counts = HashMap::<&str, u32>::new();
    #[cfg(feature = "parallel")]
    let word_counts = DashMap::<&str, u32>::new();

    #[cfg(not(feature = "parallel"))]
    let iter = words.iter();
    #[cfg(feature = "parallel")]
    let iter = words.par_iter();

    let start = Instant::now();

    iter.for_each(|word| {
        let count = word_counts.get(word).as_deref().unwrap_or(&0) + 1;
        word_counts.insert(word, count);
    });

    let duration = start.elapsed();

    println!("{:#?}", word_counts);
    println!("{:#?}", duration);
}
