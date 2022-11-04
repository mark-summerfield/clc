// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

import (
	"sort"
)

type strSet map[string]bool

func (me strSet) elements() []string {
	elements := mapKeys(me)
	sort.Strings(elements)
	return elements
}

func strSetFromSlice(s []string) strSet {
	set := strSet{}
	for _, key := range s {
		if key != "" {
			set[key] = true
		}
	}
	return set
}

func (me strSet) add(s string) {
	if s != "" {
		me[s] = true
	}
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
