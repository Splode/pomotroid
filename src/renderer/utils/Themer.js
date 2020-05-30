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
   * @param string themeName The name of the theme.
   */
  apply(themeName) {
    const theme = this.themes.find(e => {
      return Object.keys(e)[0] === themeName
    })[themeName]
    for (const k in theme) {
      document.documentElement.style.setProperty(k, theme[k])
    }
  }

  /**
   * Load themes from theme files.
   */
  _load() {
    // TODO check dir existence
    const dir = join(__dirname, './../assets/themes')
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
