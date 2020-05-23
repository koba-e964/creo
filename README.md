# creo

`creo` helps you do pretty much anything about creating problems, especially on AtCoder, such as testing, generating, validating.

## Directory structure
`creo` expects the following directory structure for each project.

```
- etc/
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
`creo init DESTINATION` will create a directory with this structure at `DESTINATION`. Missing intermediate directories will be automatically created.