#!/bin/sh

DIR=$(readlink -f "$(dirname "$0")/..")

printf "TODO: Setup link here\n"
printf "Enter Android SDK install dir: "
read ANDROID_HOME
export NDK_HOME=$ANDROID_HOME/ndk-bundle

mkdir "$DIR/NDK"
cd "$DIR/NDK"

$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir arm64
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm --install-dir arm
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch x86 --install-dir x86

