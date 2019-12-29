[![Build Status](https://travis-ci.org/KristonCosta/verbose-adventure.svg?branch=master)](https://travis-ci.org/KristonCosta/verbose-adventure)

### What is this?
Just another terminal emulator while learning about OpenGL and Rust.

Initially followed the blog series on: 
`http://nercury.github.io`

To look at how to nicely integrate Rust with OpenGL bindings.
Then going to be going through: 
`https://learnopengl.com` to pick up more OpenGL basics. 

Then lastly going to do the actual font bitmap and rendering.

### Building 

```shell script
cargo build 
```

### Running 

```shell script
cargo run
```

### Autocomplete isn't working for GL
Build the `lib/gl` project
```shell script
cd lib/gl
cargo build
``` 
and move the `bindings.rs` file from `target/debug/build/gl-*/out/` into the `lib/gl/src` folder.

### What are the game controls?

'i' to open the inventory 

'wasd' to move

'g' to pick up an item on the ground

bump into enemies to attack them

#### Legend
'o' is an orc 
'T' is a troll 
'#' is an item you can grab 
