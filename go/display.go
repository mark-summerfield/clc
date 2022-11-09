// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"fmt"
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
	// TODO
	if d.Seconds() > 0.1 {
		fmt.Println(d)
	}
}

func displayFull(dataForLang dataForLangMap, fileData []*fileDatum,
	sortByLines bool, maxWidth int) {
}
