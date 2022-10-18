// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::config::Config;
use crate::consts;
use crate::FileData;
use std::{cmp::Ordering, collections::HashMap, time::Instant};

pub fn display_summary(results: Vec<FileData>, config: Config, t: Instant) {
    let width = consts::DATA_FOR_LANG
        .get()
        .keys()
        .map(|k| k.len())
        .reduce(|acc, value| if acc >= value { acc } else { value });
    // TODO refactor pop & conversion of these HashMaps to Vecs
    let mut total_for_lang: HashMap<String, usize> = HashMap::new();
    let mut count_for_lang: HashMap<String, usize> = HashMap::new();
    for result in results {
        total_for_lang
            .entry(result.lang.clone())
            .and_modify(|lines| *lines += result.lines)
            .or_insert(result.lines);
        count_for_lang
            .entry(result.lang.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut totals: Vec<(&str, usize)> = total_for_lang
        .iter()
        .map(|(lang, lines)| (lang.as_str(), *lines))
        .collect();
    if config.sortbylines {
        totals.sort_by(|a, b| {
            let (alang, alines) = a;
            let (blang, blines) = b;
            if alines != blines {
                alines.partial_cmp(blines).unwrap_or(Ordering::Equal)
            } else {
                alang
                    .to_lowercase()
                    .partial_cmp(&blang.to_lowercase())
                    .unwrap_or(Ordering::Equal)
            }
        });
    } else {
        totals.sort_by(|a, b| {
            let (alang, alines) = a;
            let (blang, blines) = b;
            alang
                .to_lowercase()
                .partial_cmp(&blang.to_lowercase())
                .unwrap_or(
                    alines.partial_cmp(blines).unwrap_or(Ordering::Equal),
                )
        });
    }
    // TODO end of refactor
    for (lang, total) in totals {
        println!("{lang} {total}");
    }

    println!("{:.3} sec", t.elapsed().as_secs_f32());
}

pub fn display_full(results: Vec<FileData>, config: Config) {
    dbg!(results); // TODO sort etc.
}
