# Clargo

Clargo is a tool (heavilly inspired by Rust's `Cargo`) for building C projects.
Clargo is a build system (not made out of necessity for another C build system).
Clargo is NOT a compiler! (have a c compiler installed).

Clargo aims to be a simple & effective incremental build system for C applications. Linking libraries & include directories are supported.

# Usage

There is 3 main things that Clargo aims to make easier.

---

Creating projects
Simply run

```sh
clargo init my_project
```

and the tool does the rest.

---

Building projects
Using the (enabled by default) incremental build system, you can check your code, or actually link the program.
Checking

```sh
clargo check
```

Building

```sh
clargo build
```

(release builds can be made either by adding `release` to the end of the check or build commands or by modifying Clargo.toml)

---

Simple configuration:
No new language you need to learn just to add a library.
Just move your .lib's, .dll's, .a's or other libraries to ./lib in a clargo project, and add the name of the library to `libs` in Clargo.toml
Example:
libmylib.a in ./lib
Clargo.toml

```sh
...
libs = ["mylib"]
...
```

or add extra flags straight to cc with the `cflags` manifest key in Clargo.toml, or swap the c compiler by changing cc!
