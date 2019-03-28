# rocket_auth
This library provides a simple authentication system to use with [Rocket][], its purpose is to get integrated to [Rocket Contrib][Rocket] as the canonical way to do authentication. The current implementation is what I was able to come up with after taking some inspiration of Bram Vandenbogaerde's [rocket-simpleauth](https://github.com/bramvdbogaerde/auth-rs) crate and also leveraging it as a starting point.  

## Checklist
So what is left to do and what was already done?

1. make it easy to use ✔
2. make it also flexible/extendible enough ✔ (for me it is, but the topic may still need some discussion)
3. get the "Approved by Sergio"-stamp

I think 1. and 2. are already ok as is. I am not sure about 3., as Sergio told the community in Issue [#8](https://github.com/SergioBenitez/Rocket/issues/8) that he will be quite picky...so lets see what will happen 😄.

## How to use
First, you know the drill, add the following lines to your `Cargo.toml`.
```toml
[dependencies]
rocket = "0.4.0"
rocket_codegen = "0.4.0"
rocket_auth = { git="https://github.com/Weasy666/rocket_auth" }
```
Another option is to download it and add it as a module to your existing project or download it and add it in your `Cargo.toml` as local crate with a relative path from your project to this project like `rocket_auth = {path = "../rocket_auth/"}`.
Then add `extern crate rocket_auth` to your `main.rs` file and begin hacking.

## Examples
You can find a basic example in the `examples/` directory.

[Rocket]: https://github.com/SergioBenitez/Rocket
