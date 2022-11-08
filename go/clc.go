// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"bytes"
	"fmt"
	"github.com/edsrzf/mmap-go"
	"io/fs"
	"os"
	"path"
	"path/filepath"
	"strings"
	"time"
)

func main() {
	config := getConfig()
	t := time.Now()
	files := getFilenames(&config)
	fileData := processFiles(files, &config)

	// DEBUG
	for _, d := range fileData {
		fmt.Println(d)
	}
	// END DEBUG

	fmt.Println(time.Since(t))
}

func getFilenames(config *config) []string {
	files := make([]string, 0)
	for name := range config.File {
		if info, err := os.Stat(name); err == nil {
			if info.IsDir() {
				_ = filepath.WalkDir(name,
					func(path string, de fs.DirEntry, err error) error {
						if err == nil && !de.IsDir() {
							if validFilename(path, config) {
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

func validFilename(name string, config *config) bool {
	base := path.Base(name)
	if config.Include.Contains(base) {
		return true
	}
	if strings.HasPrefix(base, ".") {
		return false
	}
	// TODO
	//fmt.Println("#######", base, name)
	return true
}

func processFiles(files []string, config *config) []fileDatum {
	fileChan := make(chan fileDatum)
	for _, filename := range files {
		go processFile(filename, config, fileChan)
	}
	fileData := make([]fileDatum, 0, len(files))
	for i := 0; i < len(files); i++ {
		datum := <-fileChan
		if datum.IsValid() {
			fileData = append(fileData, datum)
		}
	}
	return fileData
}

func processFile(filename string, config *config, out chan fileDatum) {
	datum := fileDatum{filename: filename, lines: -1} // invalid
	file, err := os.Open(filename)
	if err != nil {
		out <- datum
		return
	}
	defer file.Close()
	mm, err := mmap.Map(file, mmap.RDONLY, 0)
	if err != nil {
		out <- datum
		return
	}
	defer func() { _ = mm.Unmap() }()
	datum.lines = bytes.Count(mm, []byte("\n"))
	out <- datum
}
