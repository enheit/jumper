# Jumper shell integration for Fish
# Add this to your ~/.config/fish/config.fish:
#   source /path/to/jumper.fish

function jumper --wraps=jumper
    command jumper $argv
    set exit_code $status
    if test $exit_code -eq 0 -a -f "$HOME/.cache/jumper/lastdir"
        set target_dir (cat "$HOME/.cache/jumper/lastdir")
        if test -d "$target_dir"
            cd "$target_dir"
        end
    end
    return $exit_code
end
