import { EventBus } from './event-bus'

export default class {
  constructor(min) {
    this.time = 0
    this.totalSeconds = min * 60
  }

  start() {
    if (!this.timerInt) {
      // we are not calling back on fixed seconds, but on fixed seconds plus this offset
      const msOffset = new Date().getMilliseconds()

      const timerLoop = () => {
        this.time += 1
        if (this.time >= this.totalSeconds) {
          this.pause()
          EventBus.$emit('timer-completed')
          // return to prevent the next timeout
          return
        } else {
          EventBus.$emit('timer-advanced', this.time, this.totalSeconds)
        }

        // compute how many ms to wait before the next call
        // we do this because the callback takes time, so calling with 1000 ms of delay each time
        // makes us lag behind after a bit
        const computedTimeout = 1000 - (new Date().getMilliseconds() - msOffset)
        this.timerInt = setTimeout(timerLoop, computedTimeout)
      }

      // first call is always in 1000 ms
      this.timerInt = setTimeout(timerLoop, 1000)
      EventBus.$emit('timer-started')
    }
  }

  pause() {
    clearTimeout(this.timerInt)
    delete this.timerInt
    EventBus.$emit('timer-paused')
  }

  reset() {
    clearTimeout(this.timerInt)
    delete this.timerInt
    this.time = 0
    EventBus.$emit('timer-reset')
  }

  resume() {
    if (!this.timerInt) {
      this.start()
      EventBus.$emit('timer-resumed')
    }
  }
}
