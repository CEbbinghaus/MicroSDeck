#!/bin/bash

has_sudo() {
    prompt=$(sudo -nv 2>&1)
    if [ $? -eq 0 ]; then
        # exit code of sudo-command is 0
        echo "has_sudo__pass_set"
    elif echo $prompt | grep -q '^sudo:'; then
        echo "has_sudo__needs_pass"
    else
        echo "no_sudo"
    fi
}

PluginName=$(basename "$PWD")

node util/updateVersion.js package.json lib/package.json

echo "Building plugin $PluginName..."

mkdir -p build

if [[ "$*" != *"--skip-backend"* ]]; then
    echo "Building backend..."
    cd backend && ./build.sh && cd ..
fi

if [[ "$*" != *"--skip-frontend"* ]]; then
    echo "Building frontend..."
    pnpm install && pnpm run build
fi

echo  "Collecting outputs into /build folder"
cp -r dist build/
cp -r bin build/
cp main.py build/
cp plugin.json build/
cp README.md build/
cp package.json build/

if [[ "$*" != *"--skip-copy"* ]]; then
    echo "Copying build folder to local plugin directory"

    sudo rm -rf /home/deck/homebrew/plugins/$PluginName
    sudo cp -r build/ /home/deck/homebrew/plugins/$PluginName
    sudo chmod 555 /home/deck/homebrew/plugins/$PluginName
fi

node util/resetVersion.js package.json lib/package.json