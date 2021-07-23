# commit-email

A tool that reminds you to commit with the correct email address

## Installation

### Arch Linux

commit-email is available on the
[AUR](https://aur.archlinux.org/packages/commit-email-git/).

### Other

Use
[cargo](https://doc.rust-lang.org/stable/cargo/getting-started/installation.html)
to install commit-email.

```sh
cargo install commit-email
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
