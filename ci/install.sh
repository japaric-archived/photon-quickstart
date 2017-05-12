set -ex

main() {
    cargo install xargo || true
    rustup component add rust-src || true

    if [ $TRAVIS_OS_NAME = osx ]; then
        brew cask install gcc-arm-embedded
    fi
}

main
