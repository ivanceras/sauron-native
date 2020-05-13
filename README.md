## WIP: Sauron Native
A rust UI library that conquers all platforms ranging from desktop to mobile devices.
An attempt to create a truly native, cross platform UI for your Rust applications.

One UI to rule them all.

Sauron Native extends the [Sauron](https://github.com/ivanceras/sauron) web framework
which is heavily inspired by [The Elm Architecture](https://guide.elm-lang.org/architecture/).
Sauron native can target multiple GUI backends.

## Currently supported backends
- GTK
- HTML
- tui

## Screenshots

![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-gtk.png)


![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-windows.png)


![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-html.png)


![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-tui.png)

## Running the examples

```bash
git submodule init
git submodule update
cd examples/cross_widget
./run_gtk.sh
./run_terminal.sh
# if you have wasm-pack installed
./start_web.sh
```

## TODO (General)
- Depracate itui, in favor of titik which is much simpler to maintain

## TODO (widgets)
- [X] button
- [x] checkbox
- [X] column ( vbox )
- [x] row ( hbox )
- [ ] container
- [x] image
- [ ] progress_bar
- [x] radio
- [ ] scrollable
- [ ] slider
- [ ] space
- [x] text
- [X] text_input (textbox)

## TODO (platforms)
- [X] Linux (gtk)
- [X] Windows ([native-windows-gui](https://github.com/gabdube/native-windows-gu))
- [ ] Mac ([appki-rs](https://github.com/ryanmcgrath/appkit-rs))
- [X] Browser(html)
- [X] TUI

 [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)
