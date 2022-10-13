#!/usr/bin/env python3
# Copyright © 2022 Mark Summerfield. All rights reserved.
# License: GPLv3

import argparse
import os
import pathlib

EXTS_FOR_LANG = {
    'cpp': {'.hpp', '.hxx', '.cpp', '.cxx'},
    'py': {'.pyw', '.py'},
    'rs': {'.rs'}
    }
EXCLUDE = {'__pycache__', 'build', 'dist', 'target'}


def main():
    config = get_config()
    for job in get_jobs(config):
        print(job)


def count_lines():
    pass


def get_jobs(config):
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
        config.file = {'.'}
    else:
        config.file = set(config.file)
    return config


if __name__ == '__main__':
    main()

'''
Alpha sorted except that .hpp comes before .cpp

     C++              222
     --------------------
     path/to/app.cpp   20
     path/to/lib.hpp   82
     path/to/lib.cpp  120
     ====================
     Python           852
     --------------------
     path/to/app.pyw   24
     path/to/lib.py   828
     ====================
     Rust             294
     --------------------
     path/to/app.rs   203
     path/to/lib.rs    91
'''
