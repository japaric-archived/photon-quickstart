set -ex

main() {
    for ex in $(ls examples/*); do
        local ex=$(basename $ex)
        ex=${ex%.*}
        xargo build --example $ex --release
    done
}

main
