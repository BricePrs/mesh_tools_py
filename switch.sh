# Simple script to switch between rust crate example exec and generated python lib

current_mode=$(cat .switch_rep/state)
if [ "$current_mode" = "python" ]; then

  mv Cargo.toml ./.switch_rep/Cargo_py.toml
  mv examples/example.py ./.switch_rep/example.py
  mv src/lib.rs ./.switch_rep/lib_py.rs

  mv ./.switch_rep/Cargo_rust.toml Cargo.toml
  mv ./.switch_rep/example.rs examples/example.rs
  mv ./.switch_rep/lib_rust.rs src/lib.rs


  echo "rust" > ./.switch_rep/state

elif [ "$current_mode" = "rust" ]; then

  mv Cargo.toml ./.switch_rep/Cargo_rust.toml
  mv examples/example.rs ./.switch_rep/example.rs
  mv src/lib.rs ./.switch_rep/lib_rust.rs

  mv ./.switch_rep/Cargo_py.toml Cargo.toml
  mv ./.switch_rep/example.py examples/example.py
  mv ./.switch_rep/lib_py.rs src/lib.rs


  echo "python" > ./.switch_rep/state
fi
