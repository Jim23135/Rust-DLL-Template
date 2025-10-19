### Notes
- Windows crate has both s and w macros for creating strings
- DllMain is the entrypoint called when the DLL is attached / loaded
- Make note of the `#[unsafe(no_mangle)]`
  - If compiled without this, functions will not have the same names assigned in code. no_mangle tells the compiler not to mangle the name
- Note that in Cargo.toml, there is a section called `[lib]`. The crate type is set to `cdylib`. `cdylib` signals that the output (a DLL) is intended to be dynamically linked into non-Rust programs.
- Mr. Rambles has created an amazingly detailed tutorial that taught me lots about DLLs in Rust and DLL injection. I highly suggest you check out his tutorial [here](https://samrambles.com/guides/window-hacking-with-rust/creating-a-dll-with-rust/index.html)
