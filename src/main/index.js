'use strict'

import { app, BrowserWindow, ipcMain } from 'electron'

// const WindowsToaster = require('node-notifier').WindowsToaster
// const notifier = new WindowsToaster()
// notifier.notify({
//   appID: 'com.splode.pomotroid',
//   title: 'foo',
//   message: 'Hello World',
//   id: 13
// }, (error, response) => {
//   if (error) {
//     console.log(error)
//   }
//   console.log(response)
// })

// const notifier = require('node-notifier')
// notifier.notify({
//   title: 'Pomotroid Round Completed',
//   message: 'Time to take a short break',
//   sound: false
// }, (err, res) => {
//   if (err) {
//     console.log(err)
//   }
// })

/**
 * Set `__static` path to static files in production
 * https://simulatedgreg.gitbooks.io/electron-vue/content/en/using-static-assets.html
 */
if (process.env.NODE_ENV !== 'development') {
  global.__static = require('path').join(__dirname, '/static').replace(/\\/g, '\\\\')
}

let mainWindow
const winURL = process.env.NODE_ENV === 'development'
  ? `http://localhost:9080`
  : `file://${__dirname}/index.html`

function createWindow () {
  mainWindow = new BrowserWindow({
    backgroundColor: '#2F384B',
    fullscreenable: false,
    frame: false,
    resizable: false,
    useContentSize: true,
    width: 360,
    height: 478
  })

  mainWindow.loadURL(winURL)

  mainWindow.on('closed', () => {
    mainWindow = null
  })
}

app.on('ready', createWindow)

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  if (mainWindow === null) {
    createWindow()
  }
})

ipcMain.on('window-close', (event, arg) => {
  mainWindow.close()
})

ipcMain.on('window-minimize', (event, arg) => {
  mainWindow.minimize()
})

/**
 * Auto Updater
 *
 * Uncomment the following code below and install `electron-updater` to
 * support auto updating. Code Signing with a valid certificate is required.
 * https://simulatedgreg.gitbooks.io/electron-vue/content/en/using-electron-builder.html#auto-updating
 */

/*
import { autoUpdater } from 'electron-updater'

autoUpdater.on('update-downloaded', () => {
  autoUpdater.quitAndInstall()
})

app.on('ready', () => {
  if (process.env.NODE_ENV === 'production') autoUpdater.checkForUpdates()
})
 */
