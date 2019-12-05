__Experimental__ Rust FFI bindings for [Amazon Lumberyard](https://aws.amazon.com/lumberyard/).

Also requires [modified Lumberyard source](https://github.com/jeikabu/lumberyard/tree/lmbr).

## "Tutorials"

- [Static libraries](https://rendered-obsolete.github.io/2019/09/30/lmbr_rust.html)
- [Editor plugin](https://rendered-obsolete.github.io/2019/10/05/lmbr_editor_rust.html)

## Replacing Bindings

```powershell
# Path to custom version of llvm/clang
PS> $env:LIBCLANG_PATH="D:\projects\llvm\build\Debug\bin"
# Path to Lumberyard source (defaults to `C:\Amazon\Lumberyard\1.21.0.0`)
PS> $env:LMBR_ROOT="D:\projects\lumberyard"
PS> cd ./lmbr_sys/
# Updates and replaces `lmbr_sys/src/bindings.rs`
PS> cargo build --features bindgen-generate
```