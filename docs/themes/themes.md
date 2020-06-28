# Pomotroid Themes

Pomotroid comes with many officially supported themes. You can also add any number of custom themes.

- [Pomotroid Themes](#pomotroid-themes)
  - [Available Themes](#available-themes)
  - [Creating a Custom Theme](#creating-a-custom-theme)

## Available Themes

These themes are available by default.

| Theme               | Main App                                                                | Timer Colors                                                            |
| ------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Andromeda           | ![Andromeda theme preview](images/andromeda_01.png)                     | ![Andromeda theme preview](images/andromeda_02.png)                     |
| Ayu Mirage          | ![Ayu Mirage theme preview](images/ayu_01.png)                          | ![Ayu Mirage theme preview](images/ayu_02.png)                          |
| City Lights         | ![City Lights theme preview](images/city-lights_01.png)                 | ![City Lights theme preview](images/city-lights_02.png)                 |
| Dracula             | ![Dracula theme preview](images/dracula_01.png)                         | ![Dracula theme preview](images/dracula_02.png)                         |
| D.Va                | ![D.Va theme preview](images/dva_01.png)                                | ![D.Va theme preview](images/dva_02.png)                                |
| GitHub              | ![GitHub theme preview](images/github_01.png)                           | ![GitHub theme preview](images/github_02.png)                           |
| Graphite            | ![Graphite theme preview](images/graphite_01.png)                       | ![Graphite theme preview](images/graphite_02.png)                       |
| Gruvbox             | ![Gruvbox theme preview](images/gruvbox_01.png)                         | ![Gruvbox theme preview](images/gruvbox_02.png)                         |
| Monokai             | ![Monokai theme preview](images/monokai_01.png)                         | ![Monokai theme preview](images/monokai_02.png)                         |
| Nord                | ![Nord theme preview](images/nord_01.png)                               | ![Nord theme preview](images/nord_02.png)                               |
| One Dark Pro        | ![One Dark Pro theme preview](images/one-dark-pro_01.png)               | ![One Dark Pro theme preview](images/one-dark-pro_02.png)               |
| Pomotroid (default) | ![Pomotroid theme preview](images/pomotroid_01.png)                     | ![Pomotroid theme preview](images/pomotroid_02.png)                     |
| Popping and Locking | ![Popping and Locking theme preview](images/popping-and-locking_01.png) | ![Popping and Locking theme preview](images/popping-and-locking_02.png) |
| Solarized Light     | ![Solarized Light theme preview](images/solarized-light_01.png)         | ![Solarized Light theme preview](images/solarized-light_02.png)         |
| Spandex             | ![Spandex theme preview](images/spandex_01.png)                         | ![Spandex theme preview](images/spandex_02.png)                         |
| Sythwave            | ![Sythwave theme preview](images/synthwave_01.png)                      | ![Sythwave theme preview](images/synthwave_02.png)                      |
| Tokyo Night Storm   | ![Tokyo Night Storm theme preview](images/tokyo-night-storm_01.png)     | ![Tokyo Night Storm theme preview](images/tokyo-night-storm_02.png)     |

## Creating a Custom Theme

Creating custom themes is simple. Themes are defined by a `json` file containing a **theme name** and several color values. Use the [theme template file](./theme-template.json) as a starting point.

```json
// theme-template.json
{
  "name": "Theme Name",
  "colors": {
    "--color-long-round": "",
    "--color-short-round": "",
    "--color-focus-round": "",
    "--color-background": "",
    "--color-background-light": "",
    "--color-background-lightest": "",
    "--color-foreground": "",
    "--color-foreground-darker": "",
    "--color-foreground-darkest": "",
    "--color-accent": ""
  }
}
```

To add your custom theme, copy your theme definition to the `themes` directory in the `appData` directory. The location of the `appData` depends on the operating system.

- `%APPDATA%` on **Windows**
- `$XDG_CONFIG_HOME` or `~/.config` on **Linux**
- `~/Library/Application Support` on **macOS**

For example, add the theme file to the following directory on Windows: `C:\Users\{User}\AppData\Roaming\pomotroid\themes`

Restart the application to see your new theme available as an option.
