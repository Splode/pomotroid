import { readdirSync, readFileSync } from 'fs'
import { join } from 'path'
import { initDirectory, userDir } from './LocalStore'

/**
 * Themer provides custom application styling.
 */
class Themer {
  constructor() {
    const localDir = join(__static, 'themes')
    const customDir = join(userDir(), 'themes')
    initDirectory(customDir)
    this.themes = []
    this._load([localDir, customDir])
  }

  /**
   * Apply a theme with the given name. The name should correspond to the
   * filename _without_ its extension. For example, 'dracula'.
   *
   * @param string themeName - The name of the theme.
   */
  apply(themeName) {
    const theme = this.getTheme(themeName)
    for (const k in theme.colors) {
      document.documentElement.style.setProperty(k, theme.colors[k])
    }
  }

  /**
   * Get a theme by name.
   *
   * @param string themeName - The name of the theme.
   */
  getTheme(themeName) {
    return this.themes.find(e => {
      return e.name === themeName
    })
  }

  /**
   * Get the name of a given theme.
   *
   * @param {object} theme - The theme.
   */
  getThemeName(theme) {
    return theme.name
  }

  /**
   * Get the value of a given theme's color property.
   *
   * @param {object} theme - The theme.
   * @param string value - The theme key to query.
   */
  getThemeValue(theme, value) {
    return theme.colors[value]
  }

  /**
   * Load themes from theme files.
   */
  _load(directories) {
    directories.forEach(d => {
      const files = readdirSync(d)
      files.forEach(f => {
        const theme = JSON.parse(readFileSync(join(d, f)))
        this.themes.push(theme)
      })
    })
  }
}

export default new Themer()
