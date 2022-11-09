// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"time"
)

func main() {
	config := getConfig()
	t := time.Now()
	files := getFilenames(&config)
	fileData := processFiles(files, &config)
	if config.summary {
		displaySummary(config.dataForLang, fileData, config.sortByLines,
			time.Since(t))
	} else {
		displayFull(config.dataForLang, fileData, config.sortByLines,
			config.maxWidth)
	}
}
