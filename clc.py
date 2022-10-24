#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import argparse
import collections
import concurrent.futures
import functools
import mmap
import os
import pathlib
import shutil
import sys
import time

__version__ = '1.1.0'

FileData = collections.namedtuple('FileData', ('lang', 'filename', 'lines'))
LangData = collections.namedtuple('LangData', ('name', 'exts'))

if sys.platform == 'win32':
    THIN = '-'
    THICK = '='
    ELLIPSIS = '...'
else:
    THIN = '─'
    THICK = '━'
    ELLIPSIS = '…'


FILE_COUNT_WIDTH = 7
LINE_COUNT_WIDTH = 11


DataForLang = {}


def main():
    config = get_config()
    t = time.monotonic()
    with concurrent.futures.ProcessPoolExecutor() as exe:
        file_data = exe.map(count_lines, get_filenames(config))
    file_data = tuple(file_data)
    if config.summary:
        display_summary(file_data, config.sortbylines, time.monotonic() - t)
    else:
        display_full(file_data, config.sortbylines, config.maxwidth)


def display_summary(file_data, sortbylines, secs):
    def bylines(pair):
        return pair[1], pair[0].lower()

    def bynames(pair):
        return pair[0].lower()

    total_for_lang = collections.defaultdict(int)
    count_for_lang = collections.defaultdict(int)
    LANG_WIDTH = max(len(value.name) for value in DataForLang.values())
    for file_datum in file_data:
        total_for_lang[file_datum.lang] += file_datum.lines
        count_for_lang[file_datum.lang] += 1
    for lang, total in sorted(total_for_lang.items(),
                              key=bylines if sortbylines else bynames):
        count = count_for_lang[lang]
        s = ' ' if count == 1 else 's'
        print(f'{DataForLang[lang].name:<{LANG_WIDTH}} '
              f'{count:{FILE_COUNT_WIDTH},d} file{s} '
              f'{total:{LINE_COUNT_WIDTH},d} lines')
    if secs > 0.1:
        print(f'{secs:.3f} sec')


def display_full(file_data, sortbylines, maxwidth):
    def bynames(datum):
        return datum.lang, datum.filename.lower(), datum.lines

    def bylines(datum):
        return datum.lang, datum.lines, datum.filename.lower()

    filename_width = get_width(file_data, maxwidth)
    row_width = filename_width + 1 + LINE_COUNT_WIDTH
    third = (filename_width // 3) - 1
    two_thirds = (third * 2) + 1
    lang = None
    count = subtotal = 0
    for file_datum in sorted(file_data,
                             key=bylines if sortbylines else bynames):
        if lang is None or lang != file_datum.lang:
            if lang is not None:
                display_subtotal(lang, count, subtotal, row_width)
                count = subtotal = 0
            lang = file_datum.lang
            name = f' {DataForLang[lang].name} '
            print(name.center(row_width, THICK))
        filename = file_datum.filename
        if len(filename) > filename_width:
            filename = filename[:third] + ELLIPSIS + filename[-two_thirds:]
        print(f'{filename:{filename_width}} '
              f'{file_datum.lines: >{LINE_COUNT_WIDTH},d}')
        subtotal += file_datum.lines
        count += 1
    if lang is not None:
        display_subtotal(lang, count, subtotal, row_width)
        print(THICK * row_width)


def get_width(file_data, maxwidth):
    width = 0
    for file_datum in file_data:
        if len(file_datum.filename) > width:
            width = len(file_datum.filename)
            if maxwidth is not None and width > maxwidth:
                return maxwidth
    return width


def display_subtotal(lang, count, subtotal, row_width):
    name = DataForLang[lang].name, 
    print(THIN * row_width)
    s = ' ' if count == 1 else 's'
    numbers = (f'{count:{FILE_COUNT_WIDTH},d} file{s} '
               f'{subtotal:{LINE_COUNT_WIDTH},d} lines')
    row_width -= len(numbers)
    print(f'{name:<{row_width}}{numbers}')


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
    for lang, lang_data in DataForLang.items():
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
                    # Python-specific optimizations not actually needed
                    for dir in dirs:
                        if len(dir) > 1 and dir.startswith('.'):
                            dirs.remove(dir)
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
    for part in path.parts:
        if len(part) > 1 and part.startswith('.'):
            return False
    if not path.suffix:
        return False
    for lang in config.language:
        if path.suffix in DataForLang[lang].exts:
            return True
    return False


def valid_dirname(config, name):
    if len(name) > 1 and name.startswith('.'):
        return False
    if os.path.basename(name) in config.exclude:
        return False
    return True


def get_config():
    DataForLang.update(DATA_FOR_LANG)
    read_config_files()
    width = shutil.get_terminal_size()[0]
    supported = ' '.join(sorted(DataForLang.keys()))
    parser = argparse.ArgumentParser(description=f'''
Counts the lines in the code files for the languages processed (ignoring .
folders).
Supported language names: {supported}.''')
    parser.add_argument('-l', '--language', nargs='*',
                        help='the languages to count [default: all known]')
    parser.add_argument(
        '-L', '--skiplanguage', nargs='*',
        help='the languages not to count, .e.g., "-L d cpp" with no "-l" '
        'means count all languages except D and C++ [default: none]')
    exclude = ' '.join(sorted(EXCLUDE))
    parser.add_argument(
        '-e', '--exclude', nargs='*',
        help='the files and folders to exclude [default: .hidden and '
        f'{exclude}]')
    parser.add_argument(
        '-i', '--include', nargs='*',
        help='the files to include (e.g., those without suffixes)')
    parser.add_argument(
        '-m', '--maxwidth', type=int, default=width,
        help='max line width to use (e.g., for redirected output) '
        '[default: terminal width or needed width if less]')
    parser.add_argument('-s', '--sortbylines', action='store_true',
                        help='sort by lines [default: sort by names]')
    parser.add_argument(
        '-S', '--summary', action='store_true',
        help='output per-language totals and total time if > 0.1 sec '
        '[default: output per-language and per-file totals]')
    parser.add_argument('-V', '--version', action='version',
                        version=f'%(prog)s {__version__}')
    parser.add_argument(
        'file', default='.', nargs='*',
        help='the files to count or the folders to recursively search '
        '[default: .]')
    config = parser.parse_args()
    if config.language is None:
        config.language = set(DataForLang.keys())
    else:
        config.language = set(config.language)
    if config.skiplanguage is not None:
        config.language -= set(config.skiplanguage)
    if config.exclude is None:
        config.exclude = set(EXCLUDE)
    else:
        config.exclude = set(config.exclude) | set(EXCLUDE)
    config.include = set(config.include) if config.include else set()
    if config.file == '.':
        config.file = {os.path.abspath('.')}
    else:
        config.file = {os.path.abspath(file) for file in config.file}
    config.maxwidth -= (LINE_COUNT_WIDTH + 2)
    return config


def read_config_files():
    for filename in (os.path.join(os.path.dirname(__file__), 'clc.dat'),
                     os.path.expanduser('~/clc.dat'),
                     os.path.expanduser('~/.config/clc.dat'),
                     os.path.join(os.getcwd(), 'clc.dat')):
        read_config_file(filename)


def read_config_file(filename):
    try:
        with open(filename, 'rt', encoding='utf-8') as file:
            for line in file:
                line = line.strip()
                if not line or line.startswith('#'): # skip blank & comments
                    continue
                parts = line.split('|', 2)
                if len(parts) == 3:
                    lang = parts[0].strip()
                    name = parts[1].strip()
                    exts = set()
                    for ext in parts[2].split():
                        if not ext.startswith('.'):
                            ext = f'.{ext}'
                        exts.add(ext)
                    DataForLang[lang] = LangData(name, exts)
                else:
                    print('ignoring invalid line from {filename!r}: {line}',
                          file=sys.stderr)
    except FileNotFoundError:
        pass # these files are optional


EXCLUDE = {'__pycache__', 'build', 'build.rs', 'CVS', 'dist', 'setup.py',
           'target'} # All .hidden folders are also excluded

DATA_FOR_LANG = { # Additions here may need additions in lang_for_line()
    'c': LangData('C', {'.h', '.c'}),
    'cpp': LangData('C++', {'.hpp', '.hxx', '.cpp', '.cxx'}),
    'd': LangData('D', {'.d'}),
    'go': LangData('Go', {'.go'}),
    'java': LangData('Java', {'.java'}),
    'jl': LangData('Julia', {'.jl'}),
    'nim': LangData('Nim', {'.nim'}),
    'pl': LangData('Perl', {'.pl', '.pm', '.PL'}),
    'py': LangData('Python', {'.pyw', '.py'}),
    'rb': LangData('Ruby', {'.rb'}),
    'rs': LangData('Rust', {'.rs'}),
    'tcl': LangData('Tcl', {'.tcl'}),
    'vala': LangData('Vala', {'.vala'}),
    }

if __name__ == '__main__':
    main()
