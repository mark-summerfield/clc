// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"bytes"
	"github.com/edsrzf/mmap-go"
	"os"
	"path/filepath"
	"strings"
)

func processFiles(files []string, config *config) []*fileDatum {
	fileChan := make(chan *fileDatum)
	for _, filename := range files {
		go processFile(filename, config, fileChan)
	}
	fileData := make([]*fileDatum, 0, len(files))
	for i := 0; i < len(files); i++ {
		datum := <-fileChan
		if datum != nil {
			fileData = append(fileData, datum)
		}
	}
	close(fileChan)
	return fileData
}

func processFile(filename string, config *config, out chan *fileDatum) {
	datum := &fileDatum{filename: filename}
	lang := langForName(filename, config.dataForLang)
	file, err := os.Open(filename)
	if err != nil {
		out <- nil
		return
	}
	defer file.Close()
	if lang == "" {
		line := make([]byte, 0, 60)
		file.Read(line)
		file.Seek(0, 0)
		lang = langForLine(string(line))
	}
	mm, err := mmap.Map(file, mmap.RDONLY, 0)
	if err != nil {
		out <- nil
		return
	}
	defer func() { _ = mm.Unmap() }()
	datum.lang = lang
	datum.lines = bytes.Count(mm, []byte("\n"))
	out <- datum
}

func langForName(name string, dataForLang dataForLangMap) string {
	ext := filepath.Ext(name)
	for lang, langData := range dataForLang {
		if langData.exts.Contains(ext) {
			return lang
		}
	}
	return ""
}

func langForLine(line string) string {
	if strings.HasPrefix(line, "#!") {
		i := strings.IndexByte(line, '\n')
		if i > -1 {
			line = line[:i]
		}
		if strings.Contains(line, "julia") {
			return "jl"
		}
		if strings.Contains(line, "perl") {
			return "pl"
		}
		if strings.Contains(line, "python") {
			return "py"
		}
		if strings.Contains(line, "ruby") {
			return "rb"
		}
		if strings.Contains(line, "tcl") {
			return "tcl"
		}
	}
	return ""
}
