
# TODO

## General
- [X] Calculate layouts for TUI backend using stretch
- Idea: Use raqote to render svg into BGRA
	- [X] then convert it to rgba then use it in nwg bitmap
	- [X] Use image crate to convert image to platform specific format: gtk
- [X] refine the events, should be similar to sauron
- [X] Get rid of scroll, and make it an attribute whether or not a widget should be scrollable
- [X] Use the tag as the hint for traversal behavior of a container
- [ ] Make the component return Cmd on the update similar to sauron

## Gtk backend
- [X] Implement header widget in gtk backend
- [ ] Implement the `gtk-ui` widgets
   - [X] menu
   - [X] header
   - [ ] tab_box

## WebUI backend
- [ ] rename `with-html` to `with-webui`
- [X] Make adding children to specific widgets customizable
- [X] Calculate the layout using stretch
   - This is needed since the percent value(eg: 50%) in css is hard to control

## Titik ui backend
- [ ] Refactor the titik crate to no return MSG on the draw
    - The callbacks should be executed as Fn similar to js
- [ ] Rethink of the dispatch


## WinUI backend ( Nwg backend )
- [ ] deal with nwg, flexboxlayout can not be nestabled
- [ ] events need to propagate

