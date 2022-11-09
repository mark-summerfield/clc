// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"github.com/mark-summerfield/gset"
)

type fileDatum struct {
	lang     string
	filename string
	lines    int
}

type dataForLangMap map[string]langData

type langData struct {
	name string
	exts gset.Set[string]
}

func newLangData(name string, exts ...string) langData {
	return langData{name: name, exts: gset.New(exts...)}
}

type langAndTotal struct {
	lang  string
	total int
}
