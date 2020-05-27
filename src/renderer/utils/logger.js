import electron from 'electron'
import os from 'os'
import path from 'path'
import winston from 'winston'

const app = electron.app || electron.remote.app
app.setAppLogsPath(path.join(app.getPath('userData'), 'logs'))
const userDataPath = app.getPath('logs')
const jsonWithTimestamp = winston.format.combine(
  winston.format.timestamp(),
  winston.format.json()
)

const logger = winston.createLogger({
  defaultMeta: { hostname: os.hostname() },
  format: jsonWithTimestamp,
  level: 'info',
  transports: [
    new winston.transports.File({
      filename: path.join(userDataPath, 'error.log'),
      level: 'error'
    }),
    new winston.transports.File({
      filename: path.join(userDataPath, 'combined.log')
    })
  ]
})

export { logger }
