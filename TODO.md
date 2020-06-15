
# TODO
- [X] Calculate layouts for TUI backend using stretch
- Idea: Use raqote to render svg into BGRA
	- [X] then convert it to rgba then use it in nwg bitmap
	- [X] Use image crate to convert image to platform specific format: gtk
- [X] refine the events, should be similar to sauron
- [ ] Get rid of scroll, and make it an attribute whether or not a widget should be scrollable
- [ ] Use the tag as the hint for traversal behavior of a container

- Nwg backend
   - [ ] deal with nwg, flexboxlayout can not be nestabled
   - [ ] events need to propagate
