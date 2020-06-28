'use strict'

import { logger } from './../renderer/utils/logger'
import { createLocalStore } from './../renderer/utils/LocalStore'
import { app, BrowserWindow, ipcMain, Tray, nativeImage } from 'electron'

const path = require('path')
const localStore = createLocalStore()

/**
 * Set `__static` path to static files in production
 * https://simulatedgreg.gitbooks.io/electron-vue/content/en/using-static-assets.html
 */
if (process.env.NODE_ENV !== 'development') {
  global.__static = path.join(__dirname, '/static').replace(/\\/g, '\\\\')
}

let mainWindow, tray
const winURL =
  process.env.NODE_ENV === 'development'
    ? 'http://localhost:9080'
    : `file://${__dirname}/index.html`

app.on('ready', () => {
  logger.info('app ready')
  createWindow()
  const minToTray = localStore.get('minToTray')
  const alwaysOnTop = localStore.get('alwaysOnTop')

  if (minToTray) {
    createTray()
  }

  // this must be set after window has been created on ubuntu 18.04
  mainWindow.setAlwaysOnTop(alwaysOnTop)
})

app.on('window-all-closed', () => {
  logger.info('quitting app...')
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

ipcMain.on('toggle-minToTray', (event, arg) => {
  if (arg) {
    createTray()
  } else {
    tray.destroy()
  }
})

ipcMain.on('window-close', (event, arg) => {
  mainWindow.close()
})

ipcMain.on('window-minimize', (event, arg) => {
  if (arg) {
    mainWindow.hide()
  } else {
    mainWindow.minimize()
  }
})

ipcMain.on('tray-icon-update', (event, image) => {
  const nativeImg = nativeImage.createFromDataURL(image)
  tray.setImage(nativeImg)
})

function getWindowPosition() {
  const windowBounds = mainWindow.getBounds()
  const trayBounds = tray.getBounds()

  // Center window horizontally below the tray icon
  const x = Math.round(trayBounds.x + (trayBounds.width / 2) - (windowBounds.width / 2))

  // Position window 4 pixels vertically below the tray icon
  const y = Math.round(trayBounds.y + trayBounds.height + 4)

  return { x: x, y: y }
}

function toggleWindow() {
  if (mainWindow === null) {
    createWindow()
  } else {
    mainWindow.isVisible() ? mainWindow.hide() : mainWindow.show()
  }
  const position = getWindowPosition()
  mainWindow.setPosition(position.x, position.y, false)
}

function createTray() {
//   const quitWindowMenuItem = { label: 'Exit', role: 'quit' }
  tray = new Tray(path.join(__static, 'icon-18.png'))
  tray.setToolTip('Pomotroid\nClick to Restore')
  //   tray.setContextMenu(Menu.buildFromTemplate([showWindowMenuItem, quitWindowMenuItem]))
  tray.on('click', () => {
    toggleWindow()
    // tray.popUpContextMenu()
  })
}

function createWindow() {
  const alwaysOnTop = localStore.get('alwaysOnTop')
  mainWindow = new BrowserWindow({
    alwaysOnTop,
    backgroundColor: '#2F384B',
    fullscreenable: false,
    frame: false,
    icon: path.join(__static, 'icon.png'),
    resizable: false,
    useContentSize: true,
    width: 280,
    height: 430,
    webPreferences: {
      backgroundThrottling: false,
      nodeIntegration: true
    }
  })

  mainWindow.loadURL(winURL)

  // send event to renderer on window restore
  mainWindow.on('restore', () => {
    mainWindow.webContents.send('win-restore')
  })

  // send event to renderer on window show
  mainWindow.on('show', () => {
    mainWindow.webContents.send('win-show')
  })

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
