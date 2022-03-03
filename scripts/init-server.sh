#!/bin/bash
set -e

mkdir -p ~/git-sync/meta
cd ~/git-sync/meta
git init >/dev/null 2>&1
echo "[[remote-repos]]" >.git-sync-server.toml
git add .
git commit -m 'init the repo' -q

set +e
