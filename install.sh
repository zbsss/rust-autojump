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
