import Timer from './Timer'

let timer

self.onmessage = function(msg) {
  switch (msg.data.event) {
    case 'create':
      handleCreate(msg.data.min)
      break
    case 'pause':
      handlePause()
      break
    case 'reset':
      handleReset()
      break
    case 'resume':
      handleResume()
      break
    case 'start':
      handleStart()
      break
    default:
      break
  }
}

// External event handlers

function handleCreate(min) {
  timer = new Timer(min)
  timer.addEventListener('complete', handleTimerComplete)
  timer.addEventListener('pause', handleTimerPause)
  timer.addEventListener('reset', handleTimerReset)
  timer.addEventListener('start', handleTimerStart)
  timer.addEventListener('tick', handleTimerTick)
}

function handlePause() {
  if (!timer) return
  timer.pause()
}

function handleReset() {
  if (!timer) return
  timer.reset()
}

function handleResume() {
  if (!timer) return
  timer.resume()
}

function handleStart() {
  if (!timer) return
  timer.start()
}

// Internal timer event handlers

function handleTimerComplete() {
  self.postMessage({ event: 'complete' })
}

function handleTimerPause() {
  self.postMessage({ event: 'pause' })
}

function handleTimerReset() {
  self.postMessage({ event: 'reset' })
}

function handleTimerStart() {
  self.postMessage({ event: 'start' })
}

function handleTimerTick(event) {
  self.postMessage({
    event: 'tick',
    elapsed: event.detail.time,
    totalSeconds: event.detail.totalSeconds
  })
}
