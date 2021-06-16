# orca

## Summury
Lightweight rootless container management tool

## Prerequisities
### Debian or Arch Linux
Execute below command to be able to separate user_namespace with non-root user  
`$ sudo sysctl -w kernel.unprivileged_userns_clone=1`

## Install orca
### Linux
#### Debian or Ubuntu
1. `$ sudo apt install curl uidmap`
2. `$ curl -L https://github.com/nomlab/orca/releases/download/0.1/orca > orca`
3. `$ chmod +x ./orca`

## How to use
1. `$ ./orca -d alpine -t latest date`

## Uninstall
1. `rm ./orca`
2. `rm -rf $HOME/.local/orca`

## Build static linked binary
1. Execute below commands at once to build static linked openssl
    1. `sudo apt install musl-tools gcc openssl`
    2. `sudo bash build_static_openssl.sh`

2. Install rust and MUSL
    1. Install Rust from [Rust homepage](https://www.rust-lang.org/tools/install)
    2. `$ rustup target add x86_64-unknown-linux-musl`

3. Build static linked binary
    1. `$ bash create_static_binary.sh`

