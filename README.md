## WIP: Sauron-native
a rust UI library that conquers all platforms ranging from desktop to mobile devices.
An attempt to create a truly native, truly cross platform UI for your rust applications.

One UI to rule them all

Sauron native extends the [sauron](https://github.com/ivanceras/sauron) web framework
which is heavily inspired by [The Elm Architecture](https://guide.elm-lang.org/architecture/).
Sauron native can target multiple GUI backends.

## Currently supported backend
- gtk
- html
- tui

## Screenshot

![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-gtk.png)


![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-html.png)


![](https://raw.githubusercontent.com/ivanceras/sauron-native/master/assets/sauron-native-tui.png)

## Running the example

```bash
git submodule init
git submodule update
cd examples/cross_widget
./run_gtk.sh
./run_terminal.sh
# if you have wasm-pack installed
./start_web.sh
```

## TODO widgets
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

 [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)
