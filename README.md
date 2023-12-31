# brint
basic rust integrals for quantum chemistry based on
[libcint](https://github.com/sunqm/libcint)

Currently, libcint itself is used via
[bindgen](https://github.com/rust-lang/rust-bindgen), but I hope one day to
rewrite it all in Rust directly. `libcint-sys` contains the generated bindings.
Eventually `brint` should be a safe Rust interface to these bindings. Right now
it only contains a Rust port of the `c_call_cartesian` example from the libcint
repository.
