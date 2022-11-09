// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"bufio"
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
	lang := langForName(filename, config.dataForLang)
	file, err := os.Open(filename)
	if err != nil {
		out <- nil
		return
	}
	defer file.Close()
	if lang == "" {
		line := make([]byte, 0, 60)
		_, err = file.Read(line)
		if err == nil {
			_, _ = file.Seek(0, 0)
		}
		lang = langForLine(string(line))
	}
	lines := 0
	nl := []byte("\n")
	mm, err := mmap.Map(file, mmap.RDONLY, 0)
	if err == nil {
		defer func() { _ = mm.Unmap() }()
		lines = bytes.Count(mm, nl)
	} else {
		reader := bufio.NewReader(file)
		buffer := make([]byte, 16384)
		for {
			n, err := reader.Read(buffer)
			if err != nil {
				break // real error or io.EOF
			}
			lines += bytes.Count(buffer[:n], nl)
		}
	}
	out <- &fileDatum{lang, filename, lines}
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
