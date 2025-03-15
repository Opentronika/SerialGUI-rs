#!/bin/bash

echo "🔧 Setting up Git hooks..."
mkdir -p .git/hooks
cp .githooks/* .git/hooks/
chmod +x .git/hooks/*
echo "✅ Hooks installed!"