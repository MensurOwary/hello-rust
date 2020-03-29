# grape

**grape** is a very basic tool that does a trivial text search on the given text file.

```
- USAGE:
     cargo run -- [OPTIONS] --file <path> --pattern <pattern>       If it is run using cargo
     grape [OPTIONS] --file <path> --pattern <pattern>              If its binary is added to the path
 
- FLAGS:
     -h, --help       Prints help information
     -V, --version    Prints version information
 
- OPTIONS:
     -c, --case-sensitive <case_sensitive>    If the search is case sensitive or not; default is true
     -f, --file <path>                        The path to the file to read
     -p, --pattern <pattern>                  The pattern to look for
```