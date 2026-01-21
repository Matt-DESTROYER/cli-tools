# cli-tools

> Note: these tools do not yet support the full set of UNIX arguments and are not 100% UNIX compatible, although that is the goal.

## Available/WIP Tools:
 - `cat` print one or more files to your console
 - `clear` clear the console screen
 - `ls` print the files/directories in a file or directory
 - `mv` move a file or folder to another location
 - `rm` remove a file (or recursively remove files to remove folders)

## Why?
I mean for starters, why not?

This project was born out of frustration when switching between Windows' Command Prompt and (WSL) Ubuntu's terminal.
I kept habitually attempting to use commands like `clear`, `ls`, `rm` and `mv` which of course don't actually exist on Windows.

I know PowerShell has aliases to enable you to use these, I should probably just use PowerShell, there's literally no reason why I don't... I suppose maybe they're not 100% UNIX compatible (don't really know, and it's not that relevant), and in theory I want this to eventually be 100% UNIX compatible.

But either way there's another reason; I wanted to learn (and _actually use_) Rust! So what better way to learn than making some actual CLI tools that I will actually use!!

Anyways, that's my spiel.
The goal of this project is incremental compatibility (functionality-wise) with UNIX, performance isn't really a consideration (if it becomes an issue I'm probably doing something wrong) at this point.
