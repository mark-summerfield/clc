#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import argparse
import collections
import concurrent.futures
import mmap
import os
import pathlib
import sys
import time

FileData = collections.namedtuple('FileData', ('lang', 'filename', 'lines'))
LangData = collections.namedtuple('LangData', ('name', 'exts'))

EXCLUDE = {'__pycache__', 'build', 'build.rs', 'dist', 'setup.py', 'target',
           '.git', '.hg'}

DATA_FOR_LANG = { # Additions here may need additions in lang_for_line()
    'c': LangData('C', {'.h', '.c'}),
    'cpp': LangData('C++', {'.hpp', '.hxx', '.cpp', '.cxx'}),
    'd': LangData('D', {'.d'}),
    'go': LangData('Go', {'.go'}),
    'java': LangData('Java', {'.java'}),
    'jl': LangData('Julia', {'.jl'}),
    'nim': LangData('Nim', {'.nim'}),
    'pl': LangData('Perl', {'.pl', '.PL'}),
    'py': LangData('Python', {'.pyw', '.py', '.pxd'}),
    'rb': LangData('Ruby', {'.rb'}),
    'rs': LangData('Rust', {'.rs'}),
    'tcl': LangData('Tcl', {'.tcl'}),
    'vala': LangData('Vala', {'.vala'}),
    }

if sys.platform == 'win32':
    THIN = '─'
    THICK = '═'
else:
    THIN = '─'
    THICK = '━'


def main():
    config = get_config()
    t = time.monotonic()
    with concurrent.futures.ProcessPoolExecutor() as exe:
        file_data = exe.map(count_lines, get_filenames(config))
    file_data = tuple(file_data)
    if config.totals:
        display_totals(file_data, time.monotonic() - t)
    else:
        display_full(file_data, config.sortbylines)


def display_totals(file_data, secs):
    total_for_lang = collections.defaultdict(int)
    count_for_lang = collections.defaultdict(int)
    width = max(len(name) for name in DATA_FOR_LANG.keys()) + 4
    for file_datum in file_data:
        total_for_lang[file_datum.lang] += file_datum.lines
        count_for_lang[file_datum.lang] += 1
    for lang, total in sorted(total_for_lang.items(),
                              key=lambda pair: (pair[0].lower())):
        count = count_for_lang[lang]
        s = ' ' if count == 1 else 's'
        print(f'{DATA_FOR_LANG[lang].name:<{width}} '
              f'{count:5,d} file{s} {total:10,d} lines')
    print(f'{secs:.3f} sec'.rjust(width))


def display_full(file_data, sortbylines):
    SIZE = 11
    NWIDTH = SIZE - 1
    width = 0
    for file_datum in file_data:
        if len(file_datum.filename) > width:
            width = len(file_datum.filename)
    lang = None
    count = subtotal = 0
    for file_datum in sorted(file_data,
                             key=bylines if sortbylines else bynames):
        if lang is None or lang != file_datum.lang:
            if lang is not None:
                display_subtotal(lang, count, subtotal, width, SIZE, NWIDTH)
                count = subtotal = 0
            lang = file_datum.lang
            name = f' {DATA_FOR_LANG[lang].name} '
            print(name.center(width + SIZE, THICK))
        print(f'{file_datum.filename:{width}} '
              f'{file_datum.lines: >{NWIDTH},d}')
        subtotal += file_datum.lines
        count += 1
    if lang is not None:
        display_subtotal(lang, count, subtotal, width, SIZE, NWIDTH)
        print(THICK * (width + SIZE))


def bynames(file_datum):
    return file_datum.lang, file_datum.filename.lower(), file_datum.lines


def bylines(file_datum):
    return file_datum.lang, file_datum.lines, file_datum.filename.lower()


def display_subtotal(lang, count, subtotal, width, size, nwidth):
    lang = DATA_FOR_LANG[lang].name
    span = width + size
    print(THIN * span)
    s = ' ' if count == 1 else 's'
    numbers = f'{count:3,d} file{s} {subtotal:{nwidth},d} lines'
    span -= len(numbers)
    print(f'{lang:<{span}}{numbers}')


def count_lines(name):
    lang = lang_for_name(name)
    if not os.path.getsize(name):
        return FileData(lang, name, 0)
    with open(name, 'rb') as file:
        if lang is None:
            lang = lang_for_line(file.readline())
            file.seek(0)
        with mmap.mmap(file.fileno(), 0, access=mmap.ACCESS_READ) as mm:
            return FileData(lang, name, mm[:].count(b'\n'))


def lang_for_line(line):
    if line.startswith(b'#!'):
        if b'julia' in line:
            return 'jl'
        if b'perl' in line:
            return 'pl'
        if b'python' in line:
            return 'py'
        if b'ruby' in line:
            return 'rb'
        if b'tcl' in line:
            return 'tcl'


def lang_for_name(name):
    ext = pathlib.Path(name).suffix
    for lang, lang_data in DATA_FOR_LANG.items():
        if ext in lang_data.exts:
            return lang


def get_filenames(config):
    for name in config.file:
        if os.path.isfile(name):
            if valid_filename(config, name):
                yield name
        elif os.path.isdir(name):
            if valid_dirname(config, name):
                for root, dirs, files in os.walk(name):
                    for file in files:
                        filename = os.path.join(root, file)
                        if valid_filename(config, filename):
                            yield filename
                    # Python-specific optimization not actually needed
                    for exclude in config.exclude:
                        try:
                            dirs.remove(exclude)
                        except ValueError:
                            pass


def valid_filename(config, name):
    path = pathlib.Path(name)
    if path.name in config.include:
        return True
    if path.name.startswith('.'):
        return False
    if set(path.parts) & config.exclude:
        return False
    if not path.suffix:
        return False
    for lang in config.language:
        if path.suffix in DATA_FOR_LANG[lang].exts:
            return True
    return False


def valid_dirname(config, name):
    if len(name) > 1 and name.startswith('.'):
        return False
    if os.path.basename(name) in config.exclude:
        return False
    return True


def get_config():
    supported = ' '.join(sorted(DATA_FOR_LANG.keys()))
    parser = argparse.ArgumentParser(description=f'''
Counts the lines in the code files for the languages processed.
Supported languages: {supported}.
''')
    parser.add_argument('-s', '--sortbylines', action='store_true',
                        help='sort by lines [default: sort by names]')
    parser.add_argument(
        '-t', '--totals', action='store_true',
        help='output only per-language totals not per file (and total '
        'time)')
    parser.add_argument('-l', '--language', nargs='*',
                        help='the languages to count [default: all known]')
    exclude = ' '.join(sorted(EXCLUDE))
    parser.add_argument(
        '-e', '--exclude', nargs='*',
        help=f'the files and folders to exclude [default: {exclude}]')
    parser.add_argument(
        '-i', '--include', nargs='*',
        help='the files and folders to include (e.g., those without '
        'suffixes)')
    parser.add_argument(
        'file', default='.', nargs='*',
        help='the files to count or the folders to recursively search '
        '[default: .]')
    config = parser.parse_args()
    if config.language is None:
        config.language = set(DATA_FOR_LANG.keys())
    else:
        config.language = set(config.language)
    if config.exclude is None:
        config.exclude = set(EXCLUDE)
    else:
        config.exclude = set(config.exclude) | set(EXCLUDE)
    config.include = set(config.include) if config.include else set()
    if config.file == '.':
        config.file = {os.path.abspath('.')}
    else:
        config.file = {os.path.abspath(file) for file in config.file}
    return config


if __name__ == '__main__':
    main()
