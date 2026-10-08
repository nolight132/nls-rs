<div align="center">

# nls

### neo-ls: a modern `ls` with useful tables
</div>

> [!NOTE]
> This is an ongoing Rust rewrite of [nls](https://github.com/nolight132/nls).
> It exists mostly to prevent my brain from dissolving in the age of AI, so
> every line here is written by me. I work on it from time to time. The plan
> is to eventually replace nls with this Rust implementation, so this repo
> is basically doomed.

## Why nls?

`nls` is a **neo-ls**: a modern file listing command built around beautiful tables, compatibility in pipes and scripts, and useful defaults like directory sizes without slowing normal usage.

Nushell's `ls` already provides this experience, but not everyone wants to switch shells — many users are happy with bash, zsh, fish, or PowerShell and just want the table layouts of `nu ls` without Nushell's programming model and compatibility tradeoffs.

`nls` exists for people who want the visual experience of modern terminal tools while keeping the workflows they already know.

It works in bash, zsh, fish, Nushell, PowerShell, and any terminal on Linux, macOS, or Windows where a normal CLI binary can run.

---

## Features

- Nushell-style tables for interactive terminal use
- Git status per listed entry
- Directory sizes shown by default
- Fast non-TTY behavior for pipes, redirects, and scripts
- `ls`-like behavior for common workflows
- Helpful suggestions when paths are mistyped
- Optional icons (enabled by default)
- Colors for files, directories, symlinks, executables, sizes, and timestamps
- JSON output for structured usage
- Works on Linux, macOS, and Windows, and (probably™) everywhere else you would want to run it
