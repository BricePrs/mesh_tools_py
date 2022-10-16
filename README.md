# mesh_tools_py

## Description
`mesh_tools` goal is to allow to display and modify meshes with simple transformations.
Currently only the display is implemented.
It is possible to import a .ply mesh(the parser was really made quickly and is not good but it is not essential).
Most of the work done concern the scene display as well as input management and camera control/displacement.

There is 2 modes, one is a simple rust lib and the other implement a python library (those two mode are not compatible so I made a .sh script to switch beetweeb the two).
To switch between the two modes simply run :
```bash
./switch
```

## Controls
 `ZQSD` and `mouse clic & drag` to move around
 
 `mouse scroll` to change camera displacement scale
 
 `tab` to change camera mode
 
 `esc` to quit


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

## Examples

![Screenshot from 2022-10-17 00-54-24](https://user-images.githubusercontent.com/44588205/196063701-0a6d7973-da23-48a8-90e4-a766d7669ddd.png)

![Screenshot from 2022-10-17 00-54-54](https://user-images.githubusercontent.com/44588205/196063695-d8acaf28-9575-42dd-b2b8-4a00f0c182af.png)

This video has a low framerate ! The render time for a frame is way less than 16ms.

[Demo.webm](https://user-images.githubusercontent.com/44588205/196064095-765b49f5-364a-450b-b37d-00ca1630f92b.webm)

