## Rline

**Rline** is a command-line tool that counts the number of lines, characters, and whitespace in files, displaying the results in a neatly formatted table sorted by file extension.

### Features
- **Line, Character, and Whitespace Counting**: Analyzes files to provide a breakdown of lines, characters, and whitespace.
- **File Extension and Directory Banning**: 
- **Recursive Mode**: 

### Modes
- **-p**: Specify the path to the directory you want to analyze.
- **-r**: Enable recursive analysis of subdirectories.
- **-h**: Display help information.
- **-e**: Ban file extensions
- **-d**: Ban directories

### Requirements
Rust installed

### Usage

1. **Compile the Code**:  
   Run the code with cargo using
   ```sh
   cargo run
   ```
  Or compile it by running
  ```sh
  cargo build
  ```
  
3. **Run the Program**:  
   Run the compiled executable, specifying any desired options:
   ```sh
   ./rline -p <path> -r
   ```
   Or just use it without any to use it on the directory you are located in(You must add the compiled program to the path first):
   ```sh
   ./rline
   ```

### Example Command
```sh
./rline -p /your/directory/path -r
```

This will analyze all supported files in the specified directory and its subdirectories, providing a detailed report.

