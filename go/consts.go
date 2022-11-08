// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: Apache-2.0

package main

const Version = "1.0.0"

const fileCountWidth = 7
const lineCountWidth = 11

type fileDatum struct {
	lang     string
	filename string
	lines    int
}

func (me fileDatum) IsValid() bool {
	return me.filename != "" && me.lines >= 0
}
