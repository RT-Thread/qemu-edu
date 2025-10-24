


          
**macro-main**

- Provides a way to use `main` in `no_std` mode by rewriting code through a procedural macro. It exposes a C ABI–style `main` entry and automatically places command or application metadata into designated linker sections so RT-Thread can discover and invoke it at startup or via the command table.

**Features**

- Entry wrapper: Generates an `extern "C"` entry function, converts C-style `argv` to `Vec<ParamItem>`, then calls the original Rust function.
- Component section: Places the generated function pointer into the `.rti_fn.4` section as a component initialization entry.
- App section: Places the generated function pointer into the `.rti_fn.6` section as an application initialization entry.
- Extended export notes (partially untested implementations):
    - Pre-initialization entry: `.rti_fn.2` section
    - Device initialization entry: `.rti_fn.3` section
    - Component initialization entry: `.rti_fn.4` section (tested)
    - Environment initialization entry: `.rti_fn.5` section
    - Application initialization entry: `.rti_fn.6` section (tested)
- MSH command export: Generates a command descriptor struct, places it into the `FSymTab` section, and stores related information in the `.rodata.name` section so it can be scanned and registered as an MSH command.

**Usage**

- Import the `macro_main` procedural macro in your Rust code.
- Add the `#[macro_main_use]` attribute to the `main` function.
- Register as a component or an application as needed.
    - Component: `component = true`, Automatically registers as a component initialization entry.
    - Application: `app = true`, Automatically registers as an application initialization entry.
- Compile the Rust code; the resulting executable will contain the `main` function entry.
- When C code needs to call Rust functions, declare the Rust function prototypes in C code and use `extern "C"` to specify the calling convention. For example: `extern "C" void rust_function_name(void);`

## References

[RUST support for rt-thread](https://github.com/rust-for-rtthread/rtt_rust)