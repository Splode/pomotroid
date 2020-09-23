import { logger } from './../utils/logger'
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
    autoStartWorkTimer: true,
    autoStartBreakTimer: true,
    minToTray: false,
    minToTrayOnClose: false,
    notifications: true,
    workRounds: 4,
    theme: null,
    tickSounds: false,
    tickSoundsDuringBreak: true,
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
    const userDataPath = userDir()
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
        logger.error(err)
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
  if (!fs.existsSync(filePath)) {
    fs.writeFile(filePath, JSON.stringify(defaults), err => {
      if (err) {
        logger.error(err)
      }
    })
    return defaults
  } else {
    try {
      return JSON.parse(fs.readFileSync(filePath))
    } catch (error) {
      logger.error(error)
      return defaults
    }
  }
}

/**
 * Create a directory at the given path if it doesn't exist.
 *
 * @export
 * @param {string} path - The directory path.
 */
export function initDirectory(path) {
  if (!fs.existsSync(path)) {
    logger.info(`creating directory: ${path}`)
    fs.mkdirSync(path)
  }
}

/**
 * Get the 'userData' directory in the current environment.
 *
 * @export
 * @returns {string} The userData path.
 */
export function userDir() {
  let path
  try {
    path = (electron.app || electron.remote.app).getPath('userData')
  } catch (error) {
    logger.errror('failed to get user direoctory', error)
  }
  return path
}
