@echo off

@REM Generate bindings for 'bindings-fabric-loader'
pushd bindings-fabric-loader
echo Generate bindings
..\scripts\java-spaghetti-gen.exe generate --config java-spaghetti.yaml --verbose > java-spaghetti-output.txt
rustfmt full-bindings.rs --edition 2024 --config-path ..\bindings-rustfmt.toml --config "max_width=125" --config "normalize_doc_attributes=false"
echo Separate bindings
form --input full-bindings.rs --outdir src >nul 2>nul
echo Format bindings
cargo fmt -- --config-path ..\bindings-rustfmt.toml
popd
