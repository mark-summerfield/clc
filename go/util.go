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

type keyType interface {
	int | string
}

func mapKeys[K keyType, V any](m map[K]V) []K {
	keys := make([]K, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	return keys
}

func mapValues[K keyType, V any](m map[K]V) []V {
	values := make([]V, 0, len(m))
	for _, v := range m {
		values = append(values, v)
	}
	return values
}
