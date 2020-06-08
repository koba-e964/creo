# creo [![Build Status](https://travis-ci.com/koba-e964/creo.svg?branch=master)](https://travis-ci.com/koba-e964/creo)

`creo` helps you do pretty much anything about creating problems, especially on AtCoder, such as testing, generating and validating. `creo` achieves them with interoperability.

## Overview
A `creo` project manages a collection of files needed to prepare for a **single problem**. `creo` provides a functionality to manage it.

`creo`'s capabilities include:
- test/validate/generate automatically
- interoperate with coworkers who do not use `creo`
  - Shell scripts that automatically run tasks (e.g. validation, generation) will be provided
  - Other than `creo.toml`, no `creo`-specific files will be created in project directories
- test/validate/generate better than provided scripts
  - Results are cached in temporary directories
  

Limitations are:
- `creo` does not support testing multiple problems in a problemset. If you want to do this, you need to do it manually (e.g. running `creo` in each subdirectory.)

## Directory structure
`creo` expects the following directory structure for each project.

```
- creo.toml (configuration file)
- etc/
  |- etc/score.txt
  |- etc/val-xxxx.cpp
  |- etc/gen-xxxx.cpp
  |- etc/output_checker.cpp
  |- etc/testlib.h
  :
  :
- in/
  |- in/1.txt
  |- in/2.txt
  :
  :
- out/
  |- out/1.txt
  |- out/2.txt
  :
  :
- sol/
  |- sol/xxxx.cpp
  |- sol/yyyy.cpp
  |- sol/zzzz-wa.cpp
  :
  :
- task/
  |- ja.md
  |- en.md
  :
  :
```

## Commands
### `creo init`
`creo init DESTINATION` will create a directory with the aforementioned structure at `DESTINATION`. Missing intermediate directories will be automatically created.
If `DESTINATION/creo.toml` already exists, the creation process will fail.

### `creo add`
`creo add TYPE OUTFILE` will add a file of the designated type.
`TYPE` can be one of the following:
- `val`: validator (in `etc/`)
- `gen`: generator (in `etc/`)
- `sol`: solution (in `sol/`)

If `TYPE` is `val` or `gen` and `testlib.h` is missing, it will be automatically added.

Available options are:
- `val`: nothing
- `gen`: nothing
- `sol`:
  - `--wa`: the solution should emit a wrong output 
  - `--tle`: the solution should fail to finish in the given time limit 

### `creo gen`
`creo gen PROJECT` will generate input data in `PROJECT`.

### `creo test`
`creo test` will test all solutions in `sol/`, checking if they behave as they are intended.

### `creo publish`
`creo publish` will publish all files in the project to the judge server.
Authentication must be given in `creo.toml`
- TODO: decide the format

### `creo check`
`creo check PROJECT` will check whether `PROJECT/creo.toml` is correct and if so, display its content.
