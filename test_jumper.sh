#!/bin/bash

echo "=== Testing Jumper Shell Integration ==="
echo ""
echo "Current directory: $(pwd)"
echo ""

# Remove old lastdir file
rm -f ~/.cache/jumper/lastdir

echo "Running jumper binary directly (will fail in non-interactive mode)..."
jumper 2>&1 || true

echo ""
echo "Checking if lastdir was created..."
if [ -f ~/.cache/jumper/lastdir ]; then
    echo "✓ lastdir file exists"
    echo "Contents: $(cat ~/.cache/jumper/lastdir)"
else
    echo "✗ lastdir file NOT created"
fi

echo ""
echo "=== Shell wrapper test ==="
echo "The wrapper function:"
type jumper
