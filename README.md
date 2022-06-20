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
    # Path to commit-email binary (Arch)
    /usr/bin/commit-email
    # Path to commit-email binary (Cargo)
    # $HOME/.cargo/bin/commit-email
  fi;
  command git $@
}
```

## Configure

If you want do adjust the config file for the `commit-email` tool, you will find the configs in the following path `~/.config/commit-email/commit-email.toml` (linux).

```toml
# List of repo urls where the tool will not set a commit email
ignore = []

# Emails the tool will use
emails = [
    'email@example.com',
    'email2@example.com'
]
```

The tool will also use your global git email, if set (e.g. in your global `.gitconfig`).

[0]: https://aur.archlinux.org/packages/commit-email-git/
[1]: #shell-script
[2]: https://doc.rust-lang.org/stable/cargo/getting-started/installation.html
