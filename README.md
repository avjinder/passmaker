`passmaker` is a cli password generator in Rust.

## Why did I make it?

Primarily, to learn Rust and get some experience with it.

A couple of years ago I switched over to using password managers to store and sync my passwords. So, insted of creating
my own passwords, I was using the password generators in Chrome or Bitwarden. The chrome password generator is not
configurable, but the bitwarden one is, but unlocking my vault every time I wanted to create a new password was getting
cumbersome. Hence, `passmaker`.

## Generating passwords
```sh
Usage: passmaker [OPTIONS]

Options:
  -l, --length <LENGTH>  password length                 [default: 24]
  -u, --upper            allow uppercase letters         [default: false]
      --lower            allow lowercase letters         [default: true]
  -s, --symbols          allow symbols                   [default: false]
  -n, --numbers          allow numbers                   [default: true]
  -c, --count <COUNT>    number of passwords to generate [default: 1]
  -h, --help             Print help
```
By default `passmaker` generates a single password. The `count` flag can be used to specify the number of passwords to generate.

```sh
passmaker --count 5

illtyuj60w6rkm8c5n3xmmp7
43r5yae056r9xoliqpzfdk6f
lio2hv6yqj3d7exlc7sx5i5w
ufl754qj3a0zhzdjwmin8wwq
agws2lilrtv4wavgcrfeqtiu
```
