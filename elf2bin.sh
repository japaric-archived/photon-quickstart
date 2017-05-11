# TODO turn this into a Rust tool

set -e

main() {
    local input=$1
    local name=$(basename $input)

    local td=$(mktemp -d)
    arm-none-eabi-objcopy -O binary $input $td/$name.bin.pre_crc

    head \
        -c $(($(stat -c %s $td/$name.bin.pre_crc) - 38)) \
        $td/$name.bin.pre_crc \
        > $td/$name.bin.no_crc

    tail -c 38 $td/$name.bin.pre_crc > $td/$name.bin.crc_block

    [ "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20280078563412" = $(xxd -p -c 500 $td/$name.bin.crc_block) ] \
        || (rm -rf $td && exit 1)

    shasum -a 256 $td/$name.bin.no_crc \
        | cut -c 1-65 \
        | xxd -r -p \
        | dd bs=1 \
             of=$td/$name.bin.pre_crc \
             seek=$(($(stat -c %s $td/$name.bin.pre_crc) - 38)) \
             conv=notrunc

    head \
        -c $(($(stat -c %s $td/$name.bin.pre_crc) - 4)) \
        $td/$name.bin.pre_crc \
        > $td/$name.bin.no_crc

    crc32 $td/$name.bin.no_crc \
        | cut -c 1-10 \
        | xxd -r -p \
        | dd bs=1 \
             of=$td/$name.bin.pre_crc \
             seek=$(($(stat -c %s $td/$name.bin.pre_crc) - 4)) \
             conv=notrunc

    mv $td/$name.bin.pre_crc $name.bin

    rm -rf $td
}

main "${@}"
