# orca

## Summury
Lightweight container management tool

## Prerequisities
### Debian
- Execute below command
`$ sudo sysctl -w kernel.unprivileged_userns_clone=1`

## Install orca
### Linux (Debian or Ubuntu)
1. `$ sudo apt install git curl uidmap`
2. `$ curl -L https://github.com/miyake13000/orca/releases/download/0.1/orca > orca`
3. `$ chmod +x ./orca`

## How to use
1. `$ ./orca -d hello-world -t latest /hello` 

## Uninstall
1. `$ rm ./orca`
2. `$ rm -rf $HOME/.local/orca`
