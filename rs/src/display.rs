// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::config::Config;
use crate::consts;
use crate::FileData;
use num_format::{SystemLocale, ToFormattedString};
use std::{cmp::Ordering, collections::HashMap, time::Instant};

type NForLang = HashMap<String, usize>;

pub fn display_summary(results: Vec<FileData>, config: Config, t: Instant) {
    let locale = SystemLocale::default().unwrap();
    let width = get_width();
    let (total_for_lang, count_for_lang) = get_maps(&results);
    let totals = get_sorted_totals(&total_for_lang, config.sortbylines);
    let data_for_lang = consts::DATA_FOR_LANG.get();
    for (lang, total) in totals {
        if let Some(count) = count_for_lang.get(lang) {
            let s = if *count == 1 { ' ' } else { 's' };
            if let Some(lang_data) = data_for_lang.get(&lang) {
                let count = count.to_formatted_string(&locale);
                let total = total.to_formatted_string(&locale);
                let name = lang_data.name;
                println!(
                    "{name:width$} {count:>7} file{s} {total:>12} lines"
                );
            }
        }
    }
    let secs = t.elapsed().as_secs_f32();
    if secs > 0.1 {
        println!("{secs:.3} sec");
    }
}

pub fn display_full(results: Vec<FileData>, config: Config) {
    //dbg!(results); // TODO sort etc.
}

fn get_width() -> usize {
    consts::DATA_FOR_LANG
        .get()
        .values()
        .map(|v| v.name.len())
        .reduce(|acc, value| if acc >= value { acc } else { value })
        .unwrap_or(10)
}

fn get_maps(results: &[FileData]) -> (NForLang, NForLang) {
    let mut total_for_lang = NForLang::new();
    let mut count_for_lang = NForLang::new();
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
    (total_for_lang, count_for_lang)
}

fn get_sorted_totals(
    total_for_lang: &NForLang,
    sortbylines: bool,
) -> Vec<(&str, usize)> {
    let mut totals: Vec<(&str, usize)> = total_for_lang
        .iter()
        .map(|(lang, lines)| (lang.as_str(), *lines))
        .collect();
    if sortbylines {
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
                .unwrap_or_else(|| {
                    alines.partial_cmp(blines).unwrap_or(Ordering::Equal)
                })
        });
    }
    totals
}
