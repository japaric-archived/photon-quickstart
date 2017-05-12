set -ex

main() {
    ( cd /tmp && cargo install xargo || true )
    rustup component add rust-src || true

    if [ $TRAVIS_OS_NAME = linux ]; then
        sudo add-apt-repository ppa:team-gcc-arm-embedded/ppa
        sudo apt-get update
        sudo apt-get install --no-install-recommends -y gcc-arm-embedded
    else
        brew cask install gcc-arm-embedded
    fi
}

main
