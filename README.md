`passmaker` is a cli password generator in Rust.

## Why did I make it?

Primarily, to learn Rust and get some experience with it.

A couple of years ago I switched over to using password managers to store and sync my passwords. So, insted of creating
my own passwords, I was using the password generators in Chrome or Bitwarden. The chrome password generator is not
configurable, but the bitwarden one is, but unlocking my vault every time I wanted to create a new password was getting
cumbersome. Hence, `passmaker`.

```sh
Usage: passmaker [OPTIONS]

Options:
  -l, --length <LENGTH>  password length [default: 24]
  -u, --upper            allow uppercase letters
      --lower            allow lowercase letters
  -s, --symbols          allow symbols
  -n, --numbers          allow numbers
  -c, --count <COUNT>    number of passwords to generate [default: 1]
  -h, --help             Print help
```
