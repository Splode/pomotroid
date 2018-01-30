const electron = require('electron')
const fs = require('fs')
const path = require('path')

export default class {
  constructor (opts) {
    const userDataPath = (electron.app || electron.remote.app).getPath('userData')
    this.path = path.join(userDataPath, opts.configName + '.json')
    this.data = parseDataFile(this.path, opts.defaults)
  }

  get (key) {
    return this.data[key]
  }

  set (key, val) {
    this.data[key] = val
    fs.writeFile(this.path, JSON.stringify(this.data), err => {
      if (err) {
        console.log(err)
      }
    })
    console.log(`wrote ${key}: ${val} to local store`)
  }

  setData (dataObj) {
    this.data = dataObj
    console.log(dataObj, this.data)
    fs.writeFile(this.path, JSON.stringify(this.data), err => {
      if (err) {
        console.log(err)
      }
    })
  }
}

function parseDataFile (filePath, defaults) {
  try {
    return JSON.parse(fs.readFileSync(filePath))
  } catch (error) {
    return defaults
  }
}
