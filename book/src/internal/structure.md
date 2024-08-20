# Project structure

losgatos is structured as a Cargo workspace, consisting of many crates. 

## Binary crates

Those should contain only logic that are tightly coupled with a binary file and platform specific.

* `kernel` - contains losgatos's kernel.

## Library crates

Most code in a project should go to one of library crates. This lets us make them platform-independent and testable.

* `devicetree` - utility library for no-std/no-alloc devicetree manipulation.