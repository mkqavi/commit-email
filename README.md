# commit-email

A tool that reminds you to commit with the correct email address

## Installation

### Arch Linux

commit-email is available on the [AUR][0]. Afterwards add the [shell script][1]
to your shell configuration.

### Other

Use
[cargo][2] to install commit-email.

```sh
cargo install commit-email
```

Afterwards add the [shell script][1] to your shell configuration.

## Shell Script

This script runs commit-email every time you run `git commit`.

```sh
function git() {
  if [ "$1" = "commit" ]
  then
    # Path to commit-email binary
    /usr/bin/commit-email
  fi;
  command git $@
}
```

[0]: https://aur.archlinux.org/packages/commit-email-git/
[1]: #shell-script
[2]: https://doc.rust-lang.org/stable/cargo/getting-started/installation.html
