# rust-autojump

## Installation
```bash
cargo install rust-autojump
```

For the script to work add the following code to your `.bash_profile` or `.zshrc`
```bash
rj() {
    search_phrase=$1
    best_match=$(rust-autojump -s $search_phrase)

    if [ $status -eq 0 ]
    then
      cd $best_match
    fi

    echo  $best_match
}

rust_jump_chpwd() {
    rust-autojump --add "$(pwd)" >/dev/null &!
}

typeset -gaU chpwd_functions
chpwd_functions+=rust_jump_chpwd
```

## Usage
```
rj <path>     
```

Eg.
```
$ rj tar
/Users/mkurleto/courses/rust/rust-autojump/target
```

