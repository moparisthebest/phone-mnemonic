#!/bin/bash
set -exo pipefail

echo "starting build for TARGET $TARGET"

export CRATE_NAME=phone-mnemonic

# cross doesn't actually support stdin/stdout pipes for some reason, skip it for now
DISABLE_TESTS=1

SUFFIX=""

echo "$TARGET" | grep -E '^x86_64-pc-windows-gnu$' >/dev/null && SUFFIX=".exe"

cross rustc --bin phone-mnemonic --target $TARGET --release

# to check how they are built
file "target/$TARGET/release/phone-mnemonic$SUFFIX"

if [ $DISABLE_TESTS -ne 1 ]
then

    test_num='1234567890'
    # first make sure it outputs the right number of numbers
    num_perms="$(echo "$test_num" | cross run --target $TARGET --release --bin phone-mnemonic)"
    if [ "$num_perms" != "102400" ]
    then
        echo "num_perms expected 102400, was: $num_perms"
        exit 1
    fi

    out_num="$(echo "$test_num" | cross run --target $TARGET --release --bin phone-mnemonic | cross run --target $TARGET --release --bin phone-mnemonic -- -r | sort -u)"
    if [ "$out_num" != "$test_num" ]
    then
        echo "out_num expected $test_num, was: $out_num"
        exit 1
    fi
fi

# if this commit has a tag, upload artifact to release
strip "target/$TARGET/release/phone-mnemonic$SUFFIX" || true # if strip fails, it's fine
mkdir -p release
mv "target/$TARGET/release/phone-mnemonic$SUFFIX" "release/phone-mnemonic-$TARGET$SUFFIX"

echo 'build success!'
exit 0
