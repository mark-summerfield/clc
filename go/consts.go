package main

// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

type dataForLangMap map[string]langData
type strSet map[string]bool

type langData struct {
	Name string
	Exts strSet
}

func newLangData(name string, exts ...string) langData {
	langData := langData{Name: name, Exts: make(strSet)}
	for _, ext := range exts {
		langData.Exts[ext] = true
	}
	return langData
}
