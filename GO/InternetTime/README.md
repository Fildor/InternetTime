#Internet time in GO

## How to build

### Linux
Either build directly with go:
`go build -o ./bin/beats ./src/main.go`
 or use task (https://taskfile.dev):  
`task build` which is defined in "Taskfile.yml"

That will create an executable file in '<base>/bin/'
