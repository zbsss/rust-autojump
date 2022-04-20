# rust-autojump
[crates.io](https://crates.io/crates/rust-autojump)

## Installation
```bash
cargo install rust-autojump
```

For the script to work add the following code to your `.bash_profile` or `.zshrc`   
or copy is to a script and run `source scriptname.sh`
```bash
rj() {
    search_phrase=$1
    best_match=$(rust-autojump -s $search_phrase)

    cd $best_match
    echo  $best_match
}

rust_jump_chpwd() {
    rust-autojump --add "$(pwd)" >/dev/null &!
}

typeset -gaU chpwd_functions
chpwd_functions+=rust_jump_chpwd
```

## Usage
autojump is a faster way to navigate your filesystem.   
It works by maintaining a database of the directories you use the most from the command line.

**Directories must be visited first before they can be jumped to.**

Eg. jump to a directory that best matches `foo`
```
rj foo     
```

