#!/bin/bash
clc -sS
unrecognized.py -q
python3 -m flake8 --ignore=W504,W503,E261,E303 .
python3 -m vulture . | grep -v 60%.confidence
cd go
go mod tidy
go fmt .
staticcheck .
go vet .
golangci-lint run
cd ..
git st
