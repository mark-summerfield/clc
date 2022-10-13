#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import argparse
import collections
import concurrent.futures
import mmap
import os
import pathlib

EXTS_FOR_LANG = {
    'cpp': {'.hpp', '.hxx', '.cpp', '.cxx'},
    'py': {'.pyw', '.py'},
    'rs': {'.rs'}
    }
EXCLUDE = {'__pycache__', 'build', 'dist', 'target'}
NAME_FOR_LANG = dict(cpp='C++', py='Python', rs='Rust')

Result = collections.namedtuple('Result', ('lang', 'name', 'lines'))


def main():
    config = get_config()
    with concurrent.futures.ProcessPoolExecutor() as exe:
        results = exe.map(count_lines, get_filenames(config))
    results = sorted(results, key=lambda r: (r[0], r[2], r[1].lower()))
    display(results)


def display(results):
    SIZE = 7
    NWIDTH = SIZE - 1
    width = 0
    for result in results:
        if len(result.name) > width:
            width = len(result.name)
    lang = None
    count = subtotal = 0
    # TODO use nice Unicode lines
    for result in sorted(results, key=lambda r: (r[0], r[2], r[1].lower())):
        if lang is None or lang != result.lang:
            if lang is not None:
                display_subtotal(count, subtotal, width, SIZE, NWIDTH)
                count = subtotal = 0
            lang = result.lang
            name = f' {NAME_FOR_LANG[lang]} '
            print(name.center(width + SIZE, '='))
        print(f'{result.name:{width}} {result.lines: >{NWIDTH},d}')
        subtotal += result.lines
        count += 1
    if lang is not None:
        display_subtotal(count, subtotal, width, SIZE, NWIDTH)
        print('=' * (width + SIZE))


def display_subtotal(count, subtotal, width, size, nwidth):
    print('-' * (width + size))
    s = ' ' if count == 1 else 's'
    title = f'Total lines ({count:,} file{s})'.ljust(width)
    print(f'{title} {subtotal: >{nwidth},d}')


def count_lines(name):
    lang = lang_for_name(name)
    if not os.path.getsize(name):
        return Result(lang, name, 0)
    with open(name, 'rb') as file:
        with mmap.mmap(file.fileno(), 0, prot=mmap.PROT_READ) as mm:
            return Result(lang, name, mm[:].count(b'\n'))


def lang_for_name(name):
    ext = pathlib.Path(name).suffix
    for lang, extensions in EXTS_FOR_LANG.items():
        if ext in extensions:
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
                    # Python-specific optimization not strictly needed
                    for exclude in config.exclude:
                        try:
                            dirs.remove(exclude)
                        except ValueError:
                            pass


def valid_filename(config, name):
    path = pathlib.Path(name)
    if path.name.startswith('.'):
        return False
    if set(path.parts) & config.exclude:
        return False
    if not path.suffix:
        return False
    for lang in config.language:
        if path.suffix in EXTS_FOR_LANG[lang]:
            return True
    return False


def valid_dirname(config, name):
    if len(name) > 1 and name.startswith('.'):
        return False
    if os.path.basename(name) in config.exclude:
        return False
    return True


def get_config():
    supported = ' '.join(sorted(EXTS_FOR_LANG.keys()))
    parser = argparse.ArgumentParser(description=f'''
Counts the lines in the code files for the languages processed.
Supported languages: {supported}.
''')
    parser.add_argument('-t', '--totals', action='store_true',
                        help='output only per-language totals not per file')
    parser.add_argument('-l', '--language', nargs='*',
                        help='the languages to count [default: all found]')
    exclude = ' '.join(sorted(EXCLUDE))
    parser.add_argument(
        '-e', '--exclude', nargs='*',
        help='the files and folders to exclude [default: '
        f'.HIDDEN {exclude}]')
    parser.add_argument(
        'file', default='.', nargs='*',
        help='the files to count or the folders to recursively search '
        '[default: .]')
    config = parser.parse_args()
    if config.language is None:
        config.language = set(EXTS_FOR_LANG.keys())
    else:
        config.language = set(config.language)
    if config.exclude is None:
        config.exclude = set(EXCLUDE)
    else:
        config.exclude = set(config.exclude) | set(EXCLUDE)
    if config.file == '.':
        config.file = {os.path.abspath('.')}
    else:
        config.file = {os.path.abspath(file) for file in config.file}
    return config


if __name__ == '__main__':
    main()
