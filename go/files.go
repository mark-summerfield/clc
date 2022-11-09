// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"github.com/mark-summerfield/gset"
	"io/fs"
	"os"
	"path"
	"path/filepath"
	"strings"
)

func getFilenames(config *config) []string {
	var files []string
	for name := range config.file {
		if info, err := os.Stat(name); err == nil {
			if info.IsDir() {
				_ = filepath.WalkDir(name,
					func(path string, de fs.DirEntry, err error) error {
						if err == nil {
							if de.IsDir() {
								if !validDirname(path, config) {
									return fs.SkipDir
								}
							} else if validFilename(path, config) {
								files = append(files, path)
							}
							return nil
						}
						return err
					})
			} else if validFilename(name, config) {
				files = append(files, name)
			}
		}
	}
	return files
}

func validDirname(name string, config *config) bool {
	return !((len(name) > 1 && strings.HasPrefix(name, ".")) ||
		config.exclude.Contains(filepath.Base(name)))
}

func validFilename(name string, config *config) bool {
	base := path.Base(name)
	if config.include.Contains(base) {
		return true
	}
	if strings.HasPrefix(base, ".") {
		return false
	}
	parts := gset.New(strings.Split(name, string(os.PathSeparator))...)
	if !parts.Intersection(config.exclude).IsEmpty() {
		return false
	}
	for part := range parts {
		if len(part) > 1 && strings.HasPrefix(part, ".") {
			return false
		}
	}
	ext := filepath.Ext(name)
	if ext == "" {
		return false
	}
	for lang := range config.language {
		langData := config.dataForLang[lang]
		if langData.exts.Contains(ext) {
			return true
		}
	}
	return false
}
