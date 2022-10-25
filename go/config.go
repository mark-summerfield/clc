package main

// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

import (
	"bufio"
	"fmt"
	"github.com/akamensky/argparse"
	"os"
	"path"
	"sort"
	"strings"
)

func getConfig() config {
	config := newConfig()
	initializeDataForLang(config.DataForLang)
	readConfigFiles(config.DataForLang)
	langs := strings.Join(getLangs(config.DataForLang), " ")
	desc := fmt.Sprintf("Counts the lines in the code files for the "+
		"languages processed (ignoring . folders). "+
		"Supported language names: %s.", langs)
	parser := argparse.NewParser("clc", desc)
	// TODO add args
	err := parser.Parse(os.Args)
	if err != nil {
		fmt.Print(parser.Usage(err))
	}
	// TODO
	return config
}

func getLangs(dataForLang dataForLangMap) []string {
	langs := make([]string, len(dataForLang))
	i := 0
	for lang := range dataForLang {
		langs[i] = lang
		i++
	}
	sort.Strings(langs)
	return langs
}

func initializeDataForLang(dataForLang dataForLangMap) {
	dataForLang["c"] = newLangData("C", ".h", ".c")
	dataForLang["cpp"] = newLangData("C++", ".hpp", ".hxx", ".cpp", ".cxx")
	dataForLang["d"] = newLangData("D", ".d")
	dataForLang["go"] = newLangData("Go", ".go")
	dataForLang["java"] = newLangData("Java", ".java")
	dataForLang["jl"] = newLangData("Julia", ".jl")
	dataForLang["nim"] = newLangData("Nim", ".nim")
	dataForLang["pl"] = newLangData("Perl", ".pl", ".pm", ".PL")
	dataForLang["py"] = newLangData("Python", ".pyw", ".py")
	dataForLang["rb"] = newLangData("Ruby", ".rb")
	dataForLang["rs"] = newLangData("Rust", ".rs")
	dataForLang["tcl"] = newLangData("Tcl", ".tcl")
	dataForLang["vala"] = newLangData("Vala", ".vala")
}

func readConfigFiles(dataForLang dataForLangMap) {
	exe, err := os.Executable()
	if err == nil {
		readConfigFile(dataForLang, path.Join(path.Dir(exe), "clc.dat"))
	}
	home, err := os.UserHomeDir()
	if err == nil {
		readConfigFile(dataForLang, path.Join(home, "clc.dat"))
		readConfigFile(dataForLang, path.Join(home, ".config/clc.dat"))
	}
	cwd, err := os.Getwd()
	if err == nil {
		readConfigFile(dataForLang, path.Join(cwd, "clc.dat"))
	}
}

func readConfigFile(dataForLang dataForLangMap, filename string) {
	file, err := os.Open(filename)
	if err != nil {
		return // ignore missing or unreadable files
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || line[0] == '#' {
			continue // skip blank lines & comments
		}
		parts := strings.SplitN(line, "|", 3)
		if len(parts) == 3 {
			lang := strings.TrimSpace(parts[0])
			name := strings.TrimSpace(parts[1])
			exts := []string{}
			for _, ext := range strings.Split(parts[2], " ") {
				if ext != "" && ext[0] != '.' {
					ext = "." + ext
				}
				exts = append(exts, ext)
			}
			dataForLang[lang] = newLangData(name, exts...)
		} else {
			fmt.Fprintf(os.Stderr, "ignoring invalid line from %s: %s",
				filename, line)
		}
	}
}

type config struct {
	Language    strSet
	Exclude     strSet
	Include     strSet
	MaxWidth    int
	SortByLines bool
	Summary     bool
	File        strSet
	DataForLang dataForLangMap
}

func newConfig() config {
	return config{DataForLang: make(dataForLangMap)}
}
