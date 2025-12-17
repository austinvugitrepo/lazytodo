# lazytodo

The lazily built todo app written in Rust

## Goals:

- open todo file
- write to the todo file
- save the contents in the todo file
- have the ability to append to the existing content when u next open

## How it will work:

- run the program
- opens main file for edit
- accept user input
- uses input and writes to main file
- save content / after first time appends to existing content
- repeat 

## Implementation:

- make a loop to ensure app runs until user types "exit", otherwise any input is writing to todo
- check for file existence and make a todo file for writing
- if user does not type "exit", write to the todo file: lazytodo.txt (will append to this file if it exists)

# Running this code:

## Preparation

```bash
git clone https://gitlab.com/austinvugitrepo/lazytodo.git
```

```bash
cd lazytodo
```

```bash
cd todo
```

## Using this application (please have rust installed otherwise check below for docker support)

```rust
cargo run
```

# Docker support:

## building the container:

```bash
docker build -t lazytodo:v0.1.6 .
```

## running the container for the first time:

```bash
docker run -it --name lazytodo lazytodo:v0.1.6
```

then follow "Using this application section" above

# Seeing your todo list in real time (please still be in todo subdirectory):

## On a second shell on Unix/Linux:

```bash
tail -f lazytodo.txt
```

## On a second shell on Windows:

```powershell
Get-Content lazytodo.txt -Wait
```

## While the Docker container is running in a different shell:

```bash
docker exec -it lazytodo /bin/bash
tail -f lazytodo.txt
```


