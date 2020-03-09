# itui

Itui is a fork of [tui-rs](https://github.com/fdehau/tui-rs), with a focus of providing the basic building block of writing
text based UI.
Itui's goal is to provide a parallel text-base widgets for sauron-native.
The complex widgets(such as charts, canvas, guages) of tui-rs has been removed in order to focus on most commonly used widgets
in a GUI and html.

## Goal
- be a backend for sauron-native
- widgets can trigger events
- events
    - [ ] mouse events
        - [ ] click
        - [ ] drag
        - [ ] hover
    - [ ] keyboard events
    - [ ] input event


## Text base button with hover/click effect


## Rounded button
  ╭──────╮    ▄▄▄▄▄▄▄▄
  │ btn  │    █ btn ██
  ╰──────╯    ▀▀▀▀▀▀▀▀

## Flat buttons
  ┌──────┐    ▄▄▄▄▄▄▄▄
  │ btn01│    █btn01██
  └──────┘    ▀▀▀▀▀▀▀▀

### Tabs:


```
  ╭──────╮______________
  │ tab1 │ tab2 │ tab2 │
  └──────┴──────┘──────┘
```

## License

[MIT](LICENSE)
