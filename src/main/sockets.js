const WebSocket = require('ws')
import { ipcMain } from 'electron'

let wss
let timerState = null

// Events
const GET_STATE = 'state'
const ROUND_CHANGE = 'roundChange'

/* Handle a round change from the renderer */
ipcMain.on(ROUND_CHANGE, (_event, round) => {
  timerState = round

  const message = {
    event: ROUND_CHANGE,
    data: {
      state: timerState
    }
  }

  sendGlobalMessage(message)
})

/**
 * Handle an incoming websocket message
 * @param {WebSocketClient} ws - The client instance
 * @param {string} data - The raw data sent to the websocket
 */
export const handleMessage = (ws, data) => {
  const parsedData = JSON.parse(data)

  switch (parsedData.event) {
    case GET_STATE:
      const response = {
        event: GET_STATE,
        data: {
          state: timerState
        }
      }

      sendMessage(ws, response)
      break
    default:
  }
}

/**
 * Send a websocket message to every connected client
 * @param {Object} data - Data to send to every connected client
 */
export const sendGlobalMessage = data => {
  const parsedData = JSON.stringify(data)

  wss.clients.forEach(client => {
    if (client.readyState === WebSocket.OPEN) {
      client.send(parsedData)
    }
  })
}

/**
 * Send a websocket message to an individual client
 * @param {WebSocketClient} ws - The client instance
 * @param {Object} data - Data to send to the client
 */
export const sendMessage = (ws, data) => {
  const parsedData = JSON.stringify(data)

  ws.send(parsedData)
}

/**
 * Initialize a local websocket instance and establish handlers
 * @param {Number} port - The port to run the websocket on
 */
export const init = port => {
  wss = new WebSocket.Server({
    port
  })

  wss.on('message', data => {
    logger.info(`New Websocket Message ${data}`)
  })

  wss.on('error', err => {
    logger.error(err)
  })

  wss.on('connection', ws => {
    logger.info('New Websocket Connection')

    ws.on('message', data => {
      handleMessage(ws, data)
    })
  })
}
