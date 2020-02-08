# job_submit_rust
Create a simple "hello world" Rust job submit plugin.

# Build and install instructions

    cargo build
    ./install.sh [path_to_prefix]

If path_to_prefix is not specified, then the plugin will be copied into the
default PluginDir location, which is /usr/local/lib/slurm (and the install
script will require sudo access).

# slurm.conf

    JobSubmitPlugins=rustraw

Currently, the plugin gets successfully loaded, but causes all jobs to fail
because job_submit and job_modify are not fleshed out yet.