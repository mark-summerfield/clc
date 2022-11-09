// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"fmt"
	"sort"
	//	"strings"
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
	for _, datum := range fileData {
		totalForLang[datum.lang] += datum.lines
		countForLang[datum.lang] += 1
	}
	langAndTotals := getLangAndTotals(totalForLang, sortByLines)
	// TODO
	if d.Seconds() > 0.1 {
		fmt.Println(d)
	}
}

func getLangAndTotals(totalForLang map[string]int,
	sortByLines bool) []langAndTotal {
	langAndTotals := make([]langAndTotal, len(totalForLang))
	for lang, total := range totalForLang {
		langAndTotals = append(langAndTotals, langAndTotal{lang, total})
	}
	sort.Slice(langAndTotals, func(i, j int) bool {
		if sortByLines {
		} else {
		}
	})
	return langAndTotals
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
func displayFull(dataForLang dataForLangMap, fileData []*fileDatum,
	sortByLines bool, maxWidth int) {
}
