#!/bin/sh
# Jumper shell integration for Bash and Zsh
# Add this to your ~/.bashrc or ~/.zshrc:
#   source /path/to/jumper.sh

j() {
    jumper "$@"
    if [ -f "$HOME/.cache/jumper/lastdir" ]; then
        cd "$(cat "$HOME/.cache/jumper/lastdir")" || return
    fi
}
