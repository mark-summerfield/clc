// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::config::Config;
use crate::consts;
use crate::FileData;
use num_format::{SystemLocale, ToFormattedString};
use std::{cmp::Ordering, collections::HashMap, time::Instant};

type NForLang = HashMap<String, usize>;

pub fn display_summary(
    file_data: Vec<FileData>,
    config: Config,
    t: Instant,
) {
    let locale = SystemLocale::default().unwrap();
    let width = get_width();
    let (total_for_lang, count_for_lang) = get_maps(&file_data);
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

pub fn display_full(mut file_data: Vec<FileData>, config: Config) {
    const SIZE: usize = 12;
    const NWIDTH: usize = SIZE - 1;
    let locale = SystemLocale::default().unwrap();
    let width = get_width();
    let third = (width / 3) - 1;
    let twothirds = third * 2;
    let mut lang = String::new();
    let mut count = 0;
    let mut subtotal = 0;
    let data_for_lang = consts::DATA_FOR_LANG.get();
    sort_file_data(&mut file_data, config.sortbylines);
    for file_datum in file_data {
        if lang.is_empty() || lang != file_datum.lang {
            if !lang.is_empty() {
                display_subtotal(
                    &lang,
                    count,
                    subtotal,
                    width + SIZE,
                    NWIDTH,
                );
                count = 0;
                subtotal = 0;
            }
            lang = file_datum.lang;
            if let Some(lang_data) = data_for_lang.get(lang.as_str()) {
                let name = format!(" {} ", lang_data.name);
                let span = width + SIZE;
                // TODO use nicer chars for Linux
                println!("{name:=^span$}");
            }
            // TODO
        }
        // TODO
    }
}

fn display_subtotal(
    lang: &str,
    count: usize,
    subtotal: usize,
    span: usize,
    nwidth: usize,
) {
    if let Some(lang_data) = consts::DATA_FOR_LANG.get().get(lang) {
        let locale = SystemLocale::default().unwrap();
        let name = lang_data.name;
        println!("{}", "-".repeat(span)); // TODO nicer char on linux
        let s = if count == 1 { ' ' } else { 's' };
        let name = lang_data.name;
        let count = count.to_formatted_string(&locale);
        let subtotal = subtotal.to_formatted_string(&locale);
        println!(
            "{lang:<span$} {count:>7} file{s} {subtotal:>nwidth$} lines"
        );
    }
}

fn get_width() -> usize {
    consts::DATA_FOR_LANG
        .get()
        .values()
        .map(|v| v.name.len())
        .reduce(|acc, value| if acc >= value { acc } else { value })
        .unwrap_or(10)
}

fn get_maps(file_data: &[FileData]) -> (NForLang, NForLang) {
    let mut total_for_lang = NForLang::new();
    let mut count_for_lang = NForLang::new();
    for file_datum in file_data {
        total_for_lang
            .entry(file_datum.lang.clone())
            .and_modify(|lines| *lines += file_datum.lines)
            .or_insert(file_datum.lines);
        count_for_lang
            .entry(file_datum.lang.clone())
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

fn sort_file_data(file_data: &mut Vec<FileData>, sortbylines: bool) {
    if sortbylines {
        file_data.sort_by(|a, b| {
            if a.lines != b.lines {
                a.lines.partial_cmp(&b.lines).unwrap_or(Ordering::Equal)
            } else {
                a.lang
                    .to_lowercase()
                    .partial_cmp(&b.lang.to_lowercase())
                    .unwrap_or(Ordering::Equal)
            }
        });
    } else {
        file_data.sort_by(|a, b| {
            a.lang
                .to_lowercase()
                .partial_cmp(&b.lang.to_lowercase())
                .unwrap_or_else(|| {
                    a.lines.partial_cmp(&b.lines).unwrap_or(Ordering::Equal)
                })
        });
    }
}
