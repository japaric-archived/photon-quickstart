set -ex

main() {
    cargo install xargo || true
    rustup component add rust-src || true

    if [ $TRAVIS_OS_NAME = linux ]; then
        sudo apt-get install --no-install-recommends -y gcc-arm-none-eabi
    else
        brew cask install gcc-arm-embedded
    fi
}

main
