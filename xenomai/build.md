Build using cross

```
cross build --target arm-unknown-linux-musleabihf --release 
```

The two most important things are that all rust code should be compiled as static lib.

here is an example for Cargo.toml

```
[lib]
name = "gpio"
crate-type = ["staticlib"]  
```

there should no been an main.rs file instead we will use lib.rs

