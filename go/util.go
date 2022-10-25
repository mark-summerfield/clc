package main

// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

import "sort"

type strSet map[string]bool

func strSetKeys(set strSet) []string {
	keys := make([]string, len(set))
	i := 0
	for key := range set {
		keys[i] = key
		i++
	}
	sort.Strings(keys)
	return keys
}

func strSetFromSlice(s []string) strSet {
	set := strSet{}
	for _, key := range s {
		set[key] = true
	}
	return set
}

type dataForLangMap map[string]langData

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
