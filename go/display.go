// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"fmt"
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
}

/*
	func sortFileData(fileData []*fileDatum, sortByLines bool) {
		sort.Slice(fileData, func(i, j int) bool {
			if sortByLines {
				if fileData[i].lines == fileData[j].lines {
					return strings.ToLower(fileData[i].filename) <
						strings.ToLower(fileData[j].filename)
				}
				return fileData[i].lines < fileData[j].lines
			} else {
				return strings.ToLower(fileData[i].filename) <
					strings.ToLower(fileData[j].filename)
			}
		})
	}
*/
