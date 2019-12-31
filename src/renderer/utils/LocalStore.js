const electron = require('electron')
const fs = require('fs')
const path = require('path')

export const defaults = generateSettings()

/**
 * Returns a default user-settings object.
 *
 * @returns {object} The default user-settings.
 */
function generateSettings() {
  return {
    alwaysOnTop: false,
    autoStartTimer: true,
    minToTray: false,
    notifications: true,
    workRounds: 4,
    timeLongBreak: 15,
    timeShortBreak: 5,
    timeWork: 25,
    volume: 100
  }
}

/**
 * Creates and returns an instance of LocalStore with defaults.
 *
 * @export
 * @returns {LocalStore} Instance of LocalStore.
 */
export function createLocalStore() {
  // copy defaults object
  return new LocalStore('user-preferences', Object.assign({}, defaults))
}

/**
 * Stores user configuration on the filesystem.
 *
 * @export
 * @class LocalStore
 */
export default class LocalStore {
  /**
   *Creates an instance of LocalStore.
   * @param {string} filename - The filename to store.
   * @param {object} data - Existing data or defaults to populate the LocalStore.
   * @memberof LocalStore
   */
  constructor(filename, data) {
    const userDataPath = (electron.app || electron.remote.app).getPath(
      'userData'
    )
    this.path = path.join(userDataPath, filename + '.json')
    this.data = parseDataFile(this.path, data)
  }

  /**
   * Retrieve the value of a key-value pair from data.
   *
   * @param {string} key - The key to access.
   * @returns {*} The accessed value.
   * @memberof LocalStore
   */
  get(key) {
    return this.data[key]
  }

  /**
   * Set and store a key-value pair in local storage data.
   *
   * @param {string} key - The key name.
   * @param {*} val - The value of the key property.
   * @memberof LocalStore
   */
  set(key, val) {
    this.data[key] = val
    fs.writeFileSync(this.path, JSON.stringify(this.data), err => {
      if (err) {
        console.log(err)
      }
    })
  }
}

/**
 * Attempts to parse a JSON data file given a filepath,
 * or returns given defaults.
 *
 * @param {string} filePath - The filepath of the file to be read.
 * @param {*} defaults - Defaults to be returned in the event of an error.
 * @returns {object|*}
 */
function parseDataFile(filePath, defaults) {
  try {
    return JSON.parse(fs.readFileSync(filePath))
  } catch (error) {
    return defaults
  }
}
