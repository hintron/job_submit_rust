# job_submit_rust
A simple Slurm job submit plugin skeleton written in Rust.

# Build and install instructions

    cargo build
    ./install.sh [path_to_prefix]

If path_to_prefix is not specified, then the plugin will be copied into the
default PluginDir location, which is _/usr/local/lib/slurm_ (and the install
script will require sudo access).

# Slurm Configuration

In _slurm.conf_, specify the plugin like so:

    JobSubmitPlugins=rust

When the slurmctld start ups, it will look for _job_submit_rust.so_ at either
_<PREFIX>/lib/slurm_ or at `<PluginDir>` as specified by slurm.conf (which is
_/usr/local/lib/slurm_ if unspecified). Make sure the install script put it in
the correct location.

# Logging

The `log_file()` macro will by default print to _job_submit_rust.log_ in the
same location as where slurmctld.log resides. The file name is set with the
`JOB_SUBMIT_FILE` const. This file will be created if it doesn't already exist,
and all prints will be appended.

The local system's date and time will also be printed, along with a custom
prefix. The prefix is set with the `LOG_PREFIX` const. Note that `log_file()`
is variadic (i.e. like `printf()`).

# Known issues

If you start the slurmctld and it loads this plugin, reinstall the plugin
with this script, and stop the slurmctld, you will get an abort with
SIGBUS. It doesn't like the fact that the file got replaced from under it.

Currently, `job_desc_msg_t` and `job_record_t` are not fully implemented. The
plan is to gradually implement the important fields and structures so that there
are no external dependencies, while leaving the unused pointers to structs as
void pointers and prepending "\_" to the field.

`fini()` is only called when the slurmctld exits if Slurm is configured with
`--enable-memory-leak-debug`.