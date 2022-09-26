# mesh_tools_py

## Requirement
 + Python3
 + pip3
 + maturin
 + rustc
 + rustup?
 + sdl2

```bash
    sudo apt install python3 pip3 rustup rustc libsdl2-dev python3-venv
    pip3 install maturin
```


## How to run

The first time you need to run
```bash
python3 -m venv .env
```


For each terminal run
```bash
source .env/bin/activate
```

To compile/run
```bash
maturin develop
python3 examples/example.py
```
