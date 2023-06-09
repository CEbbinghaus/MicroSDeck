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

echo "--- Building a new encrypted USDPL plugin for Decky loader ---"
echo "This script assumes you have a functioning cargo (Rust) and pnpm (Node/Javascript) setup"
echo "If you do not, parts of this script will not work correctly (but may still exit 0)"

mkdir -p build

if [[ "$*" != *"--skip-backend"* ]]; then
    # export USDPL_ENCRYPTION_KEY=$(openssl enc -aes-256-cbc -k caylon -pbkdf2 -P -md sha1 | awk -F= '{if ($1 == "key") print $2}')
    # echo "Key generated..."
    #echo USDPL key: $USDPL_ENCRYPTION_KEY

    echo "Building backend..."
    cd ./backend && ./build.sh decky && cd ..

    echo "Rebuilding USDPL frontend..."
    cd ./src/usdpl-front && ./rebuild.sh decky && cd ../..
fi

if [[ "$*" != *"--skip-frontend"* ]]; then
    echo "Building frontend..."
    # pnpm does not like local dependencies, and doesn't install them unless forced to install everything
    rm -rf ./node_modules && pnpm install && pnpm run build
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
    PluginName="DeckyPlugin"

    sudo rm -rf /home/deck/homebrew/plugins/$PluginName
    sudo cp -r build/ /home/deck/homebrew/plugins/$PluginName
    sudo chmod 555 /home/deck/homebrew/plugins/$PluginName
fi