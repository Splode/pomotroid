import { readdirSync, readFileSync } from 'fs'
import { join } from 'path'

// TODO load user themes

/**
 * Themer provides custom application styling.
 */
class Themer {
  constructor() {
    this.themes = []
    this._load()
  }

  /**
   * Apply a theme with the given name. The name should correspond to the
   * filename _without_ its extension. For example, 'dracula'.
   *
   * @param string themeName - The name of the theme.
   */
  apply(themeName) {
    const theme = this.getTheme(themeName)
    for (const k in theme) {
      document.documentElement.style.setProperty(k, theme[k])
    }
  }

  /**
   * Get a theme by name.
   *
   * @param string themeName - The name of the theme.
   */
  getTheme(themeName) {
    return this.themes.find(e => {
      return Object.keys(e)[0] === themeName
    })[themeName]
  }

  /**
   * Get the name of a given theme.
   *
   * @param {object} theme - The theme.
   */
  getThemeName(theme) {
    return Object.keys(theme)[0]
  }

  /**
   * Get the value of a given theme's property.
   *
   * @param {object} theme - The theme.
   * @param string value - The theme key to query.
   */
  getThemeValue(theme, value) {
    return theme[this.getThemeName(theme)][value]
  }

  /**
   * Load themes from theme files.
   */
  _load() {
    // TODO check dir existence
    const dir = join(__static, 'themes')
    const files = readdirSync(dir)
    files.forEach(f => {
      const filename = f.replace('.json', '')
      const data = JSON.parse(readFileSync(join(dir, f)))
      const theme = {}
      theme[filename] = data
      this.themes.push(theme)
    })
  }
}

export default new Themer()
