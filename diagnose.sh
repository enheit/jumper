#!/bin/bash

echo "=== Jumper Diagnostic ==="
echo ""

echo "1. Checking if jumper binary exists..."
which jumper
echo ""

echo "2. Checking if jumper is a function or binary..."
type jumper
echo ""

echo "3. Checking if shell integration is sourced in .bashrc..."
grep -n "jumper.sh" ~/.bashrc
echo ""

echo "4. Checking if lastdir file exists..."
if [ -f ~/.cache/jumper/lastdir ]; then
    echo "✓ File exists: ~/.cache/jumper/lastdir"
    echo "  Contents: $(cat ~/.cache/jumper/lastdir)"
else
    echo "✗ File does NOT exist: ~/.cache/jumper/lastdir"
fi
echo ""

echo "=== Instructions ==="
echo "If 'jumper' shows as a binary (not a function), run:"
echo "  hash -r"
echo "  source ~/.bashrc"
echo ""
echo "Then try running jumper in a real terminal (not through this script)"
