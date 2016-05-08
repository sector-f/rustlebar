#rustlebar
This only works with bspwm, as workspace information is received from `bspc`.

It also requires [xtitle](https://github.com/baskerville/xtitle) to work correctly.

##Configuration
Configuration can be done with a `rustlebar.toml` file. This file is looked for in the `XDG_CONFIG_HOME` directory, if this environment variable is set. If the `XDG_CONFIG_HOME` environment variable is not set, the `rustlebar.toml` file is looked for in `$HOME/.config/`

An example `rustlebar.toml` file is shown below:

````
[icons]
occupied_focused = "O"
occupied_unfocused = "o"
free_focused = "F"
free_unfocused = "f"
urgent_focused = "U"
urgent_unfocused = "u"
````

Note that the values shown above are *not* the default values; the default values are U+F111 ("" ) for occupied desktops and  U+F10C ("") for unoccupied desktops. Those probably won't show up on this page, but they are a [solid circle](http://fortawesome.github.io/Font-Awesome/icon/circle/) and a [hollow circle](http://fortawesome.github.io/Font-Awesome/icon/circle-o/) in Font Awesome.
