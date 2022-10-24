// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

package main

import (
	"fmt"
	"github.com/akamensky/argparse"
	"os"
)

func main() {
	parser := argparse.NewParser("clc",
		`Counts the lines in the code files for the languages processed (ignoring . folders). Supported language names: TODO.`)
	// TODO add args
	err := parser.Parse(os.Args)
	if err != nil {
		fmt.Print(parser.Usage(err))
	}
	fmt.Println("clc")
}
