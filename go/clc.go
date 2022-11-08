// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"time"
)

func main() {
	config := getConfig()
	fileDataChan := make(chan fileDatum)
	t := time.Now()
	for name := range config.File {
		if info, err := os.Stat(name); err == nil {
			if info.IsDir() {
				_ = filepath.WalkDir(name,
					func(path string, de fs.DirEntry, err error) error {
						if err == nil && !de.IsDir() {
							go process(name, &config, fileDataChan)
							return nil
						}
						return err
					})
			} else {
				go process(name, &config, fileDataChan)
			}
		}
	}
	fileData := make([]fileDatum, 0)
	for datum := range fileDataChan {
		fileData = append(fileData, datum)
		fmt.Println(datum)
	}
	close(fileDataChan)
	// TODO
	//fmt.Println(config)
	fmt.Println(time.Since(t))
}

func process(filename string, config *config, out chan fileDatum) {
	datum := fileDatum{filename: filename}
	out <- datum
}
