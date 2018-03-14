'use strict'

import { createLocalStore } from './../renderer/utils/local-store'
import { app, BrowserWindow, ipcMain, Tray } from 'electron'

const localStore = createLocalStore()

/**
 * Set `__static` path to static files in production
 * https://simulatedgreg.gitbooks.io/electron-vue/content/en/using-static-assets.html
 */
if (process.env.NODE_ENV !== 'development') {
  global.__static = require('path').join(__dirname, '/static').replace(/\\/g, '\\\\')
}

let mainWindow, tray
const winURL = process.env.NODE_ENV === 'development'
  ? `http://localhost:9080`
  : `file://${__dirname}/index.html`

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

ipcMain.on('toggle-alwaysOnTop', (event, arg) => {
  mainWindow.setAlwaysOnTop(arg)
})

ipcMain.on('window-close', (event, arg) => {
  mainWindow.close()
})

ipcMain.on('window-minimize', (event, arg) => {
  if (arg) {
    createTray()
    mainWindow.hide()
  } else {
    mainWindow.minimize()
  }
})

function createTray () {
  tray = new Tray('./static/icon.png')
  tray.setToolTip('Pomotroid\nClick to Restore')
  tray.on('click', () => {
    mainWindow.show()
    tray.destroy()
  })
}

function createWindow () {
  const alwaysOnTop = localStore.get('alwaysOnTop')
  mainWindow = new BrowserWindow({
    alwaysOnTop,
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
