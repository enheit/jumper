#!/bin/sh
# Jumper shell integration for Bash and Zsh
# Add this to your ~/.bashrc or ~/.zshrc:
#   source /path/to/jumper.sh

jumper() {
    command jumper "$@"
    local exit_code=$?
    if [ $exit_code -eq 0 ] && [ -f "$HOME/.cache/jumper/lastdir" ]; then
        local target_dir
        target_dir="$(cat "$HOME/.cache/jumper/lastdir")"
        if [ -d "$target_dir" ]; then
            cd "$target_dir" || return
        fi
    fi
    return $exit_code
}
