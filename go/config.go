package main

// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

import (
	"bufio"
	"flag"
	"fmt"
	//	tsize "github.com/kopoli/go-terminal-size"
	"os"
	"path"
	"sort"
	"strings"
)

const fileCountWidth = 7
const lineCountWidth = 11

func getConfig() config {
	/*
		excludes := strSet{"__pycache__": true, "build": true,
			"build.rs": true, "CVS": true, "dist": true, "setup.py": true,
			"target": true}
		dataForLang := make(dataForLangMap)
		initializeDataForLang(dataForLang)
		readConfigFiles(dataForLang)
			allLangs := getLangs(dataForLang)
			desc := fmt.Sprintf("Counts the lines in the code files for the "+
				"languages processed (ignoring . folders). "+
				"Supported language names: %s.", strings.Join(allLangs, " "))
		exclude_desc := fmt.Sprintf("the files and folders "+
			"to exclude. Default: .hidden and %s",
			strings.Join(strSetKeys(excludes), " "))
	*/
	var language flagStrings
	flag.Var(&language, "language",
		"The languages to count [default: all known]")
	flag.Var(&language, "l", "(short for -language)")
	sortByLines := flag.Bool("bylines", false,
		"Sort by lines [default: sort by names]")
	flag.Parse()

	// DEBUG
	fmt.Println("language", language)
	fmt.Println("sortbylines", *sortByLines)
	/*
		parser := argparse.NewParser("clc", desc)
		language := parser.StringList("l", "language", &argparse.Options{
			Required: false,
			Help:     "The languages to count. Default: all known"})
		skiplanguage := parser.StringList("L", "skiplanguage",
			&argparse.Options{
				Required: false,
				Help: "The languages not to count, " +
					"e.g., '-L d -L cpp' with no '-l' means count all " +
					"languages except D and C++. Default: none"})
		exclude := parser.StringList("e", "exclude", &argparse.Options{
			Required: false,
			Help: fmt.Sprintf("the files and folders to exclude. "+
				"Default: .hidden and %s",
				strings.Join(strSetKeys(excludes), " "))})
		include := parser.StringList("i", "include", &argparse.Options{
			Required: false,
			Help: "The files and folders to include (e.g., those without " +
				"suffixes"})
		width := 80
		size, err := tsize.GetSize()
		if err == nil {
			width = size.Width
		}
		maxWidth := parser.Int("m", "maxwidth", &argparse.Options{
			Required: false,
			Help:     "Max line width to use (e.g., for redirected output)",
			Default:  width})
		sortByLines := parser.Flag("s", "sortbylines", &argparse.Options{
			Required: false,
			Help:     "Sort by lines. Default: sort by names"})
		summary := parser.Flag("S", "summary", &argparse.Options{
			Required: false,
			Help: "Output per-language totals and total time if > 0.1 " +
				"sec. Default: output per-language and per-file totals"})
		// TODO FIXME
		file := parser.StringPositional(&argparse.Options{
			Required: false,
			Help:     "The files to count or the folders to recursively search",
		})
		err = parser.Parse(os.Args)
		if err != nil {
			fmt.Print(parser.Usage(err))
		}
		//fmt.Println(parser.GetArgs())
		fmt.Printf("########### %v\n", *file) // TODO
		langs := strSet{}
		if len(*language) == 0 {
			langs = strSetFromSlice(allLangs)
		} else {
			langs = strSetFromSlice(*language)
		}
		if len(*skiplanguage) > 0 {
			for _, lang := range *skiplanguage {
				delete(langs, lang)
			}
		}
		for _, excl := range *exclude {
			excludes[excl] = true
		}
		config := config{
			Language:    langs,
			Exclude:     excludes,
			Include:     strSetFromSlice(*include),
			MaxWidth:    *maxWidth - (lineCountWidth + 2),
			SortByLines: *sortByLines,
			Summary:     *summary,
			//File:        strSetFromSlice(*file), // TODO
			DataForLang: dataForLang}
	*/
	config := config{} // TODO
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
			exts := strings.Split(parts[2], " ")
			for i := 0; i < len(exts); i++ {
				if exts[i] != "" && exts[i][0] != '.' {
					exts[i] = "." + exts[i]
				}
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
