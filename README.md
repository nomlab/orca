# orca

## Summury
Lightweight rootless container management tool

## Prerequisities
### Debian or Arch Linux
Execute below command to be able to separate user_namespace with non-root user.  
1. `$ sudo sysctl -w kernel.unprivileged_userns_clone=1`

## Install orca
### Linux
1. `$ curl -L https://github.com/miyake13000/orca/releases/latest/download/orca > orca`
2. `$ chmod +x ./orca`

### Optional
We recommend to install uidmap package.  
If uidmap is not installed, you cannot create new user in container.  
1. `$ sudo apt install uidmap`

## How to use
1. `$ ./orca -d hello-world -t latest /hello`

## Uninstall
1. `rm ./orca`
2. `rm -rf $HOME/.local/orca`

## Build static linked binary
1. Execute below commands at once to build static linked openssl
    1. `sudo apt install musl-tools gcc openssl`
    2. `sudo bash ibuild_static_openssl.sh`

2. Install rust and MUSL
    1. Install Rust from [Rust homepage](https://www.rust-lang.org/tools/install)
    2. `$ rustup target add x86_64-unknown-linux-musl`

3. Build static linked binary
    1. `$ bash create_static_binary.sh`

