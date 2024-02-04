# Hop

![GitHub Release](https://img.shields.io/github/v/release/ssanj/hop)

Hop to your frequently used directories.

## Installation

## Downloading a Release

1. Download the latest [release](https://github.com/ssanj/hop/releases) for your operating system (linux or macos).
2. Make it executable with:

```
chmod +x <HOP_EXEC>
```

3. Copy executable to a directory on your path.


### Building from Source

Run:

```
cargo build --release
```

Copy binary file from `target/release/hop` to a directory on your path.


## Usage

Run `hop -h` for options:

```
Hop 0.1.5
Sanj Sahayam
Hop to frequently used directories

USAGE:
    hop [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -l, --list       Lists hoppable directories
    -t, --table      tabulate hoppable directories
    -V, --version    Prints version information

OPTIONS:
    -c, --c <HOP_HOME>          Absolute path to the hop home directory. Defaults to ~/.hop if not specified
    -d, --delete <NAME>         Delete a named directory
    -j, --jump <NAME>           Jump to a named directory
    -m, --mark <NAME> <PATH>    Mark a named directory
```

### Marking Directories

Before you can hop to a directory, you need to `mark` it. You can mark a directory with:

```
hop -m <NAME> <PATH>
```

For example:

```
hop -m code /path/to/my/code/dir
```

This will do two things:

1. Create a directory called `~/.hop` if it does not exist. If you want to a different home directory see [Changing the Hop Home Directory](#changing-hop-home-directory)
1. Create a symlink in `~/.hop` called `code` which points to `/path/to/my/code/dir`

### Listing Marks

You can list your marks with `hop -l`:

```
code
```

or list your marks and the target directories with `hop -t`:

```
code -> /path/to/my/code/dir
```

### Jump to Marks

You can get the target directory for a mark with `hop -j`:

```
hop -j code
```

which results in

```
/path/to/my/code/dir
```

You can then hook that up to `cd` to change to the above directory:


```
cd $(hop -j code)
```

### Deleting Marks

You can delete a mark with `hop -d`:

```
hop -d code
```

### Changing Hop Home Directory

If you want hop home to be another directory other than `~/.hop`, you can set that up by using `-c <new_config_dir>` when calling any command.

For example:

```
hop -c /path/to/my/hop/home -l
```
