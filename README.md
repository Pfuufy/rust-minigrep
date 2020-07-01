# Minigrep CLI program
This is a replica of the grep bash program written in Rust.

## To run
Pop open a terminal and run `cargo run <search string> <filename>`. The program will print any lines to the console that have the string you're searching for.

## Additional options
You can specify case sensitivity. The default is that the search is case sensitive. To make it case insensitive, there are two options:

- Add the optional parameter `false` after the search string and file name
- Define the environment variable `CASE_INSENSITIVE`. If you want to specify this every time you run the program, run `CASE_INSENSITIVE=1 cargo run <search string> <filename>`.
