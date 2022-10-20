// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::config::Config;
use crate::consts;
use crate::types::FileData;
use crate::util;
use num_format::{SystemLocale, ToFormattedString};
use std::{cmp::Ordering, collections::HashMap, time::Instant};

type NForLang = HashMap<String, usize>;

pub fn display_summary(
    file_data: Vec<FileData>,
    config: Config,
    t: Instant,
) {
    let lang_width = get_lang_width();
    let (total_for_lang, count_for_lang) = get_maps(&file_data);
    let totals = get_sorted_totals(&total_for_lang, config.sortbylines);
    for (lang, total) in totals {
        if let Some(count) = count_for_lang.get(lang) {
            display_summary_line(*count, total, lang, lang_width);
        }
    }
    let secs = t.elapsed().as_secs_f32();
    if secs > 0.1 {
        println!("{secs:.3} sec");
    }
}

fn get_lang_width() -> usize {
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

fn display_summary_line(
    count: usize,
    total: usize,
    lang: &str,
    lang_width: usize,
) {
    let locale = SystemLocale::default().unwrap();
    let count_width = consts::FILE_COUNT_WIDTH;
    let lines_width = consts::LINE_COUNT_WIDTH;
    let s = if count == 1 { ' ' } else { 's' };
    if let Some(lang_data) = consts::DATA_FOR_LANG.get().get(&lang) {
        let count = count.to_formatted_string(&locale);
        let total = total.to_formatted_string(&locale);
        let name = &lang_data.name;
        println!(
            "{name:lang_width$} {count:>count_width$} file{s} \
            {total:>lines_width$} lines"
        );
    }
}

pub fn display_full(mut file_data: Vec<FileData>, config: Config) {
    let locale = SystemLocale::default().unwrap();
    let filename_width = get_filename_width(&file_data, config.maxwidth);
    let row_width = filename_width + 1 + consts::LINE_COUNT_WIDTH;
    let third = (filename_width / 3) - 1;
    let lines_width = consts::LINE_COUNT_WIDTH;
    let mut lang = String::new();
    let mut count = 0;
    let mut subtotal = 0;
    sort_file_data(&mut file_data, config.sortbylines);
    for file_datum in file_data {
        if lang.is_empty() || lang != file_datum.lang {
            (lang, count, subtotal) = display_new_lang(
                &lang,
                count,
                subtotal,
                row_width,
                &file_datum,
            );
        }
        let filename =
            util::elide(&file_datum.filename, third, filename_width);
        let lines = file_datum.lines.to_formatted_string(&locale);
        println!("{filename:filename_width$} {lines: >lines_width$}");
        subtotal += file_datum.lines;
        count += 1;
    }
    if !lang.is_empty() {
        display_subtotal(&lang, count, subtotal, row_width);
        #[cfg(windows)]
        println!("{}", "=".repeat(row_width));
        #[cfg(unix)]
        println!("{}", "━".repeat(row_width));
    }
}

fn get_filename_width(file_data: &[FileData], maxwidth: usize) -> usize {
    if let Some(width) = file_data
        .iter()
        .map(|f| f.filename.len())
        .reduce(|acc, value| if acc >= value { acc } else { value })
    {
        width
    } else {
        maxwidth
    }
    .min(maxwidth)
}

fn sort_file_data(file_data: &mut [FileData], sortbylines: bool) {
    if sortbylines {
        file_data.sort_by(|a, b| {
            if a.lang != b.lang {
                a.lang.partial_cmp(&b.lang).unwrap_or(Ordering::Equal)
            } else if a.lines != b.lines {
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
            if a.lang != b.lang {
                a.lang.partial_cmp(&b.lang).unwrap_or(Ordering::Equal)
            } else {
                a.filename
                    .to_lowercase()
                    .partial_cmp(&b.filename.to_lowercase())
                    .unwrap_or_else(|| {
                        a.lines
                            .partial_cmp(&b.lines)
                            .unwrap_or(Ordering::Equal)
                    })
            }
        });
    }
}

fn display_new_lang(
    lang: &str,
    count: usize,
    subtotal: usize,
    row_width: usize,
    file_datum: &FileData,
) -> (String, usize, usize) {
    let mut zeros = false;
    if !lang.is_empty() {
        display_subtotal(lang, count, subtotal, row_width);
        zeros = true;
    }
    let lang = &file_datum.lang;
    if let Some(lang_data) = consts::DATA_FOR_LANG.get().get(lang.as_str())
    {
        let name = format!(" {} ", lang_data.name);
        #[cfg(windows)]
        println!("{name:=^row_width$}");
        #[cfg(unix)]
        println!("{name:━^row_width$}");
    }
    if zeros {
        (lang.to_string(), 0, 0)
    } else {
        (lang.to_string(), count, subtotal)
    }
}

fn display_subtotal(
    lang: &str,
    count: usize,
    subtotal: usize,
    row_width: usize,
) {
    if let Some(lang_data) = consts::DATA_FOR_LANG.get().get(lang) {
        let name = lang_data.name;
        let locale = SystemLocale::default().unwrap();
        #[cfg(windows)]
        println!("{}", "-".repeat(row_width));
        #[cfg(unix)]
        println!("{}", "─".repeat(row_width));
        let s = if count == 1 { ' ' } else { 's' };
        let count = count.to_formatted_string(&locale);
        let subtotal = subtotal.to_formatted_string(&locale);
        let count_width = consts::FILE_COUNT_WIDTH;
        let lines_width = consts::LINE_COUNT_WIDTH;
        let numbers = format!(
            "{count:>count_width$} file{s} {subtotal:>lines_width$} lines"
        );
        let width = (row_width - numbers.len()).min(10000);
        println!("{name:<width$}{numbers}");
    }
}
