
# TODO
- [X] Calculate layouts for TUI backend using stretch
    - [ ] The layout calculation becomes redundant, since TUI, GTK, and html has it's own way of dealing with calculation
        - The stretch calculation here, needs to be removed
- Idea: Use raqote to render svg into BGRA
	- [X] then convert it to rgba then use it in nwg bitmap
	- [X] Use image crate to convert image to platform specific format: gtk
- [X] refine the events, should be similar to sauron
- [X] Get rid of scroll, and make it an attribute whether or not a widget should be scrollable
- [X] Use the tag as the hint for traversal behavior of a container
- [X] Implement header widget in gtk backend

- Nwg backend
   - [ ] deal with nwg, flexboxlayout can not be nestabled
   - [ ] events need to propagate
- Web backend
   - [C] rename `with-html` to `with-webui`
   - [X] Make adding children to specific widgets customizable
   - [X] Calculate the layout using stretch
       - This is needed since the percent (50%) in css is hard to control
   - [ ] Implement the `gtk-ui` equivalent
       - [ ] menu
       - [ ] header
       - [ ] tab_box
- [ ] Make component return Cmd
- [ ] Add styling support for gtk widgets
- [ ] Make the conversion of a widget based on trait
    - this allows creating custom widget from without having to be incorporated into the core logic code
- [ ] translate the style such as FlexDirection to "flex-direction"

# Internal TODO:
- convert sauron event into web_sys Event, so the user can control the event stop_propagation, and prevent_default

# TODO widgets
- [ ] Box
- [ ] Pane
- [ ] GroupBox
- [ ] Editor
    - [ ] text editor ( full text editor, such as gtksourceview )
    - [ ] Searchbox
    - [ ] Forms
    - [ ] Dataviewer ( make a demo of data-viewer, which works in html, gtk, and titik)


## Wishlist
- [ ] femtovg backend
