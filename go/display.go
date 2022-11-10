// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"fmt"
	"github.com/mark-summerfield/gong"
	"golang.org/x/text/message"
	"sort"
	"strings"
	"time"
	"unicode/utf8"
)

func displaySummary(dataForLang dataForLangMap, fileData []*fileDatum,
	sortByLines bool, d time.Duration) {
	totalForLang := make(map[string]int, len(dataForLang))
	countForLang := make(map[string]int, len(dataForLang))
	langWidth := 0
	for _, langData := range dataForLang {
		if size := utf8.RuneCountInString(langData.name); size > langWidth {
			langWidth = size
		}
	}
	lineFmt := fmt.Sprintf("%%-%ds %%%dd file%%s %%%dd lines\n", langWidth,
		fileCountWidth, lineCountWidth)
	for _, datum := range fileData {
		totalForLang[datum.lang] += datum.lines
		countForLang[datum.lang] += 1
	}
	langAndTotals := getLangAndTotals(totalForLang, sortByLines)
	out := message.NewPrinter(message.MatchLanguage("en"))
	for _, datum := range langAndTotals {
		count := countForLang[datum.lang]
		s := "s"
		if count == 1 {
			s = " "
		}
		langName := dataForLang[datum.lang].name
		out.Printf(lineFmt, langName, count, s, datum.total)
	}
	if d.Seconds() > 0.1 {
		fmt.Printf("%0.3f sec\n", d.Seconds())
	}
}

func getLangAndTotals(totalForLang map[string]int,
	sortByLines bool) []langAndTotal {
	langAndTotals := make([]langAndTotal, 0, len(totalForLang))
	for lang, total := range totalForLang {
		langAndTotals = append(langAndTotals, langAndTotal{lang, total})
	}
	sort.Slice(langAndTotals, func(i, j int) bool {
		if sortByLines {
			if langAndTotals[i].total == langAndTotals[j].total {
				return strings.ToLower(langAndTotals[i].lang) <
					strings.ToLower(langAndTotals[j].lang)
			}
			return langAndTotals[i].total < langAndTotals[j].total
		} else {
			return strings.ToLower(langAndTotals[i].lang) <
				strings.ToLower(langAndTotals[j].lang)
		}
	})
	return langAndTotals
}

func displayFull(dataForLang dataForLangMap, fileData []*fileDatum,
	sortByLines bool, maxWidth int) {
	filenameWidth := getWidth(fileData, maxWidth)
	rowWidth := filenameWidth + 1 + lineCountWidth
	lang := ""
	count := 0
	subtotal := 0
	filenameFmt := fmt.Sprintf("%%-%ds %%%dd\n", filenameWidth,
		lineCountWidth)
	out := message.NewPrinter(message.MatchLanguage("en"))
	sortFileData(fileData, sortByLines)
	for _, datum := range fileData {
		if lang == "" || lang != datum.lang {
			if lang != "" {
				displaySubtotal(dataForLang[lang].name, count, subtotal,
					rowWidth)
				count = 0
				subtotal = 0
			}
			name := " " + dataForLang[datum.lang].name + " "
			fmt.Println(gong.Centered(name, []rune(thick)[0], rowWidth))
		}
		lang = datum.lang
		filename := gong.ElideMiddle(datum.filename, filenameWidth)
		out.Printf(filenameFmt, filename, datum.lines)
		subtotal += datum.lines
		count++
	}
	if lang != "" {
		displaySubtotal(dataForLang[lang].name, count, subtotal,
			rowWidth)
		fmt.Println(strings.Repeat(thick, rowWidth))
	}
}

func sortFileData(fileData []*fileDatum, sortByLines bool) {
	sort.Slice(fileData, func(i, j int) bool {
		if sortByLines {
			if fileData[i].lang != fileData[j].lang {
				return fileData[i].lang < fileData[j].lang
			}
			if fileData[i].lines == fileData[j].lines {
				return strings.ToLower(fileData[i].filename) <
					strings.ToLower(fileData[j].filename)
			}
			return fileData[i].lines < fileData[j].lines
		} else {
			if fileData[i].lang != fileData[j].lang {
				return fileData[i].lang < fileData[j].lang
			}
			return strings.ToLower(fileData[i].filename) <
				strings.ToLower(fileData[j].filename)
		}
	})
}

func getWidth(fileData []*fileDatum, maxWidth int) int {
	width := 0
	for _, datum := range fileData {
		size := utf8.RuneCountInString(datum.filename)
		if size > width {
			width = size
			if maxWidth > 0 && width > maxWidth {
				return maxWidth
			}
		}
	}
	return width
}

func displaySubtotal(name string, count, subtotal, rowWidth int) {
	fmt.Println(strings.Repeat(thin, rowWidth))
	s := "s"
	if count == 1 {
		s = " "
	}
	numFmt := fmt.Sprintf("%%%dd file%s %%%dd lines", fileCountWidth, s,
		lineCountWidth)
	out := message.NewPrinter(message.MatchLanguage("en"))
	numbers := out.Sprintf(numFmt, count, subtotal)
	rowWidth -= utf8.RuneCountInString(numbers)
	rowFmt := fmt.Sprintf("%%-%ds%%s\n", rowWidth)
	fmt.Printf(rowFmt, name, numbers)
}
