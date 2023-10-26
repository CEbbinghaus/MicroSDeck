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

echo "Building plugin $PluginName..."

mkdir -p build

if [[ "$*" != *"--skip-backend"* ]]; then
    echo "Building backend..."
    cd backend && ./build.sh && cd ..
fi

if [[ "$*" != *"--skip-lib"* ]]; then
    echo "Building library..."
    # pnpm does not like local dependencies, and doesn't install them unless forced to install everything
    cd lib && pnpm install && pnpm run build && cd ..
fi

if [[ "$*" != *"--skip-frontend"* ]]; then
    echo "Building frontend..."
    # pnpm does not like local dependencies, and doesn't install them unless forced to install everything
    cd frontend && pnpm install && pnpm run build && cd ..
fi

echo  "Collecting outputs into /build folder"
cp -r frontend/dist build/
cp -r bin build/
cp main.py build/
cp plugin.json build/
cp README.md build/
cp frontend/package.json build/

if [[ "$*" != *"--skip-copy"* ]]; then
    echo "Copying build folder to local plugin directory"

    sudo rm -rf /home/deck/homebrew/plugins/$PluginName
    sudo cp -r build/ /home/deck/homebrew/plugins/$PluginName
    sudo chmod 555 /home/deck/homebrew/plugins/$PluginName
fi
