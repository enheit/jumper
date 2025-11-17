# Jumper shell integration for Fish
# Add this to your ~/.config/fish/config.fish:
#   source /path/to/jumper.fish

function j
    jumper $argv
    if test -f "$HOME/.cache/jumper/lastdir"
        cd (cat "$HOME/.cache/jumper/lastdir")
    end
end
