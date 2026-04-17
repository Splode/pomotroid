# Pomotroid Themes

Pomotroid ships with 38 built-in themes and supports an unlimited number of user-created custom themes. Custom themes are hot-reloaded — no restart required.

## Built-in themes

Andromeda, Ayu (Mirage), Catppuccin Frappé, Catppuccin Latte, Catppuccin Macchiato, Catppuccin Mocha, City Lights, Cobalt2, Crimson White, Darcula, Dracula, D.Va, Everforest, GitHub, GitHub Dark, Graphite, Gruvbox, Gruvbox Light, Horizon, Kanagawa, Material Palenight, Monokai, Monokai Pro, Night Owl, Nord, One Dark Pro, Panda, Pomotroid (default dark), Pomotroid Light (default light), Popping and Locking, Rosé Pine, Rosé Pine Dawn, Rosé Pine Moon, Solarized Dark, Solarized Light, Spandex, Synthwave, Tokyo Night

## Creating a custom theme

A theme is a single `.json` file with a name and a set of hex color values.

### 1. Create the themes directory

The directory is not created automatically. Make it once:

**Linux**

```sh
mkdir -p ~/.local/share/com.splode.pomotroid/themes
```

**macOS**

```sh
mkdir -p ~/Library/Application\ Support/com.splode.pomotroid/themes
```

**Windows** (PowerShell)

```powershell
New-Item -ItemType Directory -Force "$env:APPDATA\com.splode.pomotroid\themes"
```

### 2. Create a theme file

Copy the template below into a `.json` file in the themes directory. The filename can be anything — the displayed name comes from the `name` field.

```json
{
  "name": "My Theme",
  "colors": {
    "--color-focus-round": "#ff4e4d",
    "--color-short-round": "#05ec8c",
    "--color-long-round": "#0bbddb",
    "--color-background": "#2f384b",
    "--color-background-light": "#3d4457",
    "--color-background-lightest": "#9ca5b5",
    "--color-foreground": "#f6f2eb",
    "--color-foreground-darker": "#c0c9da",
    "--color-foreground-darkest": "#dbe1ef",
    "--color-accent": "#05ec8c"
  }
}
```

### 3. Select your theme

Open **Settings → Appearance**. Your theme appears in the picker with a **Custom** badge. Select it for the Light or Dark slot (or both).

## Color reference

| Key                           | Used for                                   |
| ----------------------------- | ------------------------------------------ |
| `--color-focus-round`         | Work round indicator — dial arc, round dot |
| `--color-short-round`         | Short break indicator                      |
| `--color-long-round`          | Long break indicator                       |
| `--color-background`          | Main window background                     |
| `--color-background-light`    | Sidebar, cards, elevated surfaces          |
| `--color-background-lightest` | Borders, dividers, subtler surfaces        |
| `--color-foreground`          | Primary text                               |
| `--color-foreground-darker`   | Secondary text, labels                     |
| `--color-foreground-darkest`  | Tertiary text, placeholders                |
| `--color-accent`              | Highlighted elements, active states        |

All values must be CSS hex colors (`#rrggbb` or `#rrggbbaa`).

## Hot-reload

Pomotroid watches the themes directory while running. Saving a file — including edits to an existing theme — updates the Appearance picker within half a second. There is no need to reopen settings or restart the app.

## Overriding a built-in theme

If a custom theme's `name` exactly matches a built-in theme name (case-insensitive), it replaces that theme in the picker. This lets you tweak an existing theme without adding a new entry to the list.

## Using bundled themes as a starting point

The bundled theme files are a useful reference. You can find them in the source repository under [`static/themes/`](./static/themes/).
