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
    let lang_width = get_width();
    let count_width = consts::FILE_COUNT_WIDTH;
    let lines_width = consts::LINE_COUNT_WIDTH;
    let (total_for_lang, count_for_lang) = get_maps(&file_data);
    let totals = get_sorted_totals(&total_for_lang, config.sortbylines);
    for (lang, total) in totals {
        if let Some(count) = count_for_lang.get(lang) {
            let s = if *count == 1 { ' ' } else { 's' };
            if let Some(lang_data) = consts::DATA_FOR_LANG.get().get(&lang)
            {
                let count = count.to_formatted_string(&locale);
                let total = total.to_formatted_string(&locale);
                let name = &lang_data.name;
                println!(
                    "{name:lang_width$} {count:>count_width$} file{s} \
                    {total:>lines_width$} lines"
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
    let mut data_for_lang: HashMap<String, consts::LangData> =
        HashMap::new();
    for (lang, lang_data) in consts::DATA_FOR_LANG.get().iter() {
        data_for_lang.insert(lang.to_string(), lang_data.clone());
    }
    let locale = SystemLocale::default().unwrap();
    let filename_width = get_width();
    let row_width = filename_width + 1 + consts::LINE_COUNT_WIDTH;
    let third = (filename_width / 3) - 1;
    let two_thirds = third * 2;
    let lines_width = consts::LINE_COUNT_WIDTH;
    let mut lang = String::new();
    let mut count = 0;
    let mut subtotal = 0;
    sort_file_data(&mut file_data, config.sortbylines);
    dbg!(1, file_data.len());
    for file_datum in file_data {
        dbg!(2, &lang, count, subtotal);
        if lang.is_empty() || lang != file_datum.lang {
            if !lang.is_empty() {
                let name = if let Some(lang_data) =
                    data_for_lang.get(lang.as_str())
                {
                    lang_data.name.to_string()
                } else {
                    "".to_string()
                };
                if !name.is_empty() {
                    dbg!(3); // FIXME never returns from this call:
                    display_subtotal(&name, count, subtotal, row_width);
                }
                count = 0;
                subtotal = 0;
            }
            lang = file_datum.lang;
            dbg!(4);
            if let Some(lang_data) = data_for_lang.get(lang.as_str()) {
                let name = format!(" {} ", lang_data.name);
                // TODO use nicer chars for Linux
                println!("{name:=^row_width$}");
            }
            let filename = file_datum.filename;
            /* TODO elide middle
            if filename.len() > filename_width {
                let mut chars1;
                let mut chars2;
                for (i, c) in filename.chars().enumerate() {
                    if i < third {
                        chars1.push(c);
                    }
                }
            }
            */
            let lines = file_datum.lines.to_formatted_string(&locale);
            println!("{filename:filename_width$} {lines: >lines_width$}");
            subtotal += file_datum.lines;
            count += 1;
            dbg!(5);
        }
    }
    if !lang.is_empty() {
        let name = if let Some(lang_data) = data_for_lang.get(lang.as_str())
        {
            lang_data.name.to_string()
        } else {
            "".to_string()
        };
        if !name.is_empty() {
            display_subtotal(&name, count, subtotal, row_width);
        }
        println!("{}", "=".repeat(row_width)); // TODO nicer char on linux
    }
}

fn display_subtotal(
    name: &str,
    count: usize,
    subtotal: usize,
    row_width: usize,
    locale: &SystemLocale,
) {
    let locale = SystemLocale::default().unwrap();
    println!("{}", "-".repeat(row_width)); // TODO nicer char on linux
    let s = if count == 1 { ' ' } else { 's' };
    let count = count.to_formatted_string(&locale);
    let subtotal = subtotal.to_formatted_string(&locale);
    let count_width = consts::FILE_COUNT_WIDTH;
    let lines_width = consts::LINE_COUNT_WIDTH;
    let numbers = format!(
        "{count:>count_width$} file{s} {subtotal:>lines_width$} lines"
    );
    let width = row_width - numbers.len();
    println!("{name:<width$}{numbers}");
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
