# tmuxa
This is a tmux wrapper writing in bash and another one writing in Rust.
The tmux wrapper is good for managed tmux session. 

# New tmuxa
the new tmuxa is writing in rust and can be installed with:
```
cargo install --git https://github.com/kepet19/tmuxa.git
```

# Features
- Support for spaces
- Create a tmux session
- deleting a tmux sesssion

# Use case preview of the rust version
```
$ tmuxa
```
![MenuTmuxa](screenshots/show_tmuxa.gif)

## Binds for the shell
This binds `Ctrl + F`

`.bashrc`
```bash
bind '"\C-f" "tmuxa\n"'
```
`.zshrc`
```zsh
bindkey -s '^f' "tmuxa\n"
```

