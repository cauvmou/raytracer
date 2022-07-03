# Rust Raytracer

This is a example project of a raytracer written in rust.
I've included 8 examples, that showcase development, step by step.

The algorithms come from a example raytracer that was written using [Processing4](https://processing.org/) (= A java library for creative programming).
This example raytracer was written by one of my professors at [HTL Rennweg](https://www.htlrennweg.at/).

Although because it was originally written in Java I had to rework how the rendering was going to take place.

## Examples

To run a example you have to have the RUST programming language install, visit it's website in order to find instructions on how to download it.
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