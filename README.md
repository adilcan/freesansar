# freesansar (as in "freebsd")

```
  █████▒██▀███  ▓█████ ▓█████   ██████  ▄▄▄       ███▄    █   ██████  ▄▄▄       ██▀███  
▓██   ▒▓██ ▒ ██▒▓█   ▀ ▓█   ▀ ▒██    ▒ ▒████▄     ██ ▀█   █ ▒██    ▒ ▒████▄    ▓██ ▒ ██▒
▒████ ░▓██ ░▄█ ▒▒███   ▒███   ░ ▓██▄   ▒██  ▀█▄  ▓██  ▀█ ██▒░ ▓██▄   ▒██  ▀█▄  ▓██ ░▄█ ▒
░▓█▒  ░▒██▀▀█▄  ▒▓█  ▄ ▒▓█  ▄   ▒   ██▒░██▄▄▄▄██ ▓██▒  ▐▌██▒  ▒   ██▒░██▄▄▄▄██ ▒██▀▀█▄  
░▒█░   ░██▓ ▒██▒░▒████▒░▒████▒▒██████▒▒ ▓█   ▓██▒▒██░   ▓██░▒██████▒▒ ▓█   ▓██▒░██▓ ▒██▒
 ▒ ░   ░ ▒▓ ░▒▓░░░ ▒░ ░░░ ▒░ ░▒ ▒▓▒ ▒ ░ ▒▒   ▓▒█░░ ▒░   ▒ ▒ ▒ ▒▓▒ ▒ ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░
 ░       ░▒ ░ ▒░ ░ ░  ░ ░ ░  ░░ ░▒  ░ ░  ▒   ▒▒ ░░ ░░   ░ ▒░░ ░▒  ░ ░  ▒   ▒▒ ░  ░▒ ░ ▒░
 ░ ░     ░░   ░    ░      ░   ░  ░  ░    ░   ▒      ░   ░ ░ ░  ░  ░    ░   ▒     ░░   ░ 
          ░        ░  ░   ░  ░      ░        ░  ░         ░       ░        ░  ░   ░     
                                                                                        
```

freesansar WILL BE an experimental microkernel written in Rust.

So my design is just a minimal impl to have the fundamental components, such as bootstrapping, cpu setup, kernel entries etc.

I only aim x86_64 vmz for now.

## Building
- It relies on some unstable rust features for now, so run `rustup override set nightly`
- run `cargo build` to compile the source
- use a x86_64 vm (e.g. qemu) for testing

## TODO

- **memory handling**
- **interruptz**
- **scheduling**
- **hardware access**
- **logging etc**
