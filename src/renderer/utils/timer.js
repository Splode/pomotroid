import { EventBus } from './event-bus'

export default class {
  constructor (min) {
    this.time = 0
    this.totalSeconds = min * 60
  }

  start () {
    if (!this.timerInt) {
      this.timerInt = setInterval(() => {
        this.time += 1
        if (this.time >= this.totalSeconds) {
          this.pause()
          EventBus.$emit('timer-completed')
        }
      }, 1000)
      EventBus.$emit('timer-started')
    }
  }

  pause () {
    clearInterval(this.timerInt)
    delete this.timerInt
    EventBus.$emit('timer-paused')
  }

  reset () {
    clearInterval(this.timerInt)
    delete this.timerInt
    this.time = 0
    EventBus.$emit('timer-reset')
  }

  resume () {
    if (!this.timerInt) {
      this.start()
      EventBus.$emit('timer-resumed')
    }
  }
}
