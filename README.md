# Rust Raytracer

This is an example project of a raytracer written in Rust.
I've included 8 examples that showcase development, step by step.

The algorithms come from an example raytracer that was written in [Processing4](https://processing.org/) (= A Java framework for creative programming).
It was written by one of my professors at [HTL Rennweg](https://www.htlrennweg.at/), here is a link to his [blog](https://herrfessa.com).

Because it was originally written in Java, I had to rework how the rendering was going to take place since I'm using Rust.
I additionally added some new features such as HDRI sampling for the background.

## Examples

To run an example you have to have the [Rust](https://www.rust-lang.org/learn/get-started) programming language installed, visit it's website in order to find instructions on how to download and install it.
After that you just have to run:
```
cargo run --release --example <EXAMPLE>
```
Replace `<EXAMPLE>` with one of the examples listed in the [Cargo.toml](Cargo.toml) file or in the [examples](examples) folder.

### 01planes
```
cargo run --release --example 01planes
```
![Example Image](out/01out.png)

### 02spheres
```
cargo run --release --example 02spheres
```
![Example Image](out/02out.png)

### 03lights
```
cargo run --release --example 03lights
```
![Example Image](out/03out.png)

### 04pointlights
```
cargo run --release --example 04pointlights
```
![Example Image](out/04out.png)

### 05diffuse
```
cargo run --release --example 05diffuse
```
![Example Image](out/05out.png)

### 06specular
```
cargo run --release --example 06specular
```
![Example Image](out/06out.png)

### 07reflection
```
cargo run --release --example 07reflection
```
![Example Image](out/07out.png)

### 08background
```
cargo run --release --example 08background
```
![Example Image](out/08out.png)
