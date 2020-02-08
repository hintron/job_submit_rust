#!/bin/sh
################################################################################
# Install the Rust job submit plugin .so into the proper place
#
# Requires the patchelf binary
################################################################################

# Input: Slurm's build prefix
prefix="$1"

name="job_submit_rustraw.so"
target="./target/debug/libjob_submit_rustraw.so"
default_plugin_dir="/usr/local/lib/slurm"
target_destination=$default_plugin_dir

if [ "$prefix" != "" ]; then
    target_destination="$prefix/lib/slurm/$name"
fi

# Cargo can't rename libs or give it an soname, so do it manually here
# https://github.com/rust-lang/cargo/issues/1970
# https://stackoverflow.com/questions/18467163/is-there-any-way-to-change-the-soname-of-a-binary-directly
patchelf --set-soname $name $target

echo "installing $target_destination"
cp "$target" "$target_destination"
