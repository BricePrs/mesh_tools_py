# mesh_tools_py

## Description
`mesh_tools` goal is to allow to display and modify meshes with simple transformations.
Currently only the display is implemented.
It is possible to import a .ply mesh(the parser was really made quickly and is not good but it is not essential).
Most of the work done concern the scene display as well as input management and camera mouvements.

There is 2 modes, one is a simple rust lib and the other implement a python library (those two mode are not compatible so I made a .sh script to switch beetweeb the two).
To switch between the two modes simply run :
```bash
./switch
```

## How to use

### Requirement
 + Python3
 + pip3
 + maturin
 + rustc
 + sdl2

Here is a quick command to install most of the lib necessary (haven't fully checked if it works)
```bash
    sudo apt install python3 python3-pip rustc libsdl2-dev python3-venv
    pip3 install maturin
```

## Rust mode
### How to run

```bash
cargo run
```

### Python mode

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
