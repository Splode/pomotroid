import { EventBus } from './event-bus'

/**
 * Creates a new instance of a Timer.
 * The Timer manages the timers for the application via setInterval.
 *
 * @export
 * @class Timer
 */
export default class Timer {
  /**
   *Creates an instance of Timer.
   * @param {number} min - The number of minutes to countdown.
   * @memberof Timer
   */
  constructor(min) {
    this.time = 0
    this.totalSeconds = min * 60
  }

  /**
   * Start the timer by creating a new timer.
   *
   * @memberof Timer
   * @emits 'timer-started' - Emitted when the timer has started.
   * @emits 'timer-advanced' - Emits current time and total time at interval.
   * @emits 'timer-completed' - Emitted when the timer has completed.
   */
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

  /**
   * Pause the timer.
   * Clears and removes current interval.
   *
   * @memberof Timer
   */
  pause() {
    clearTimeout(this.timerInt)
    delete this.timerInt
    EventBus.$emit('timer-paused')
  }

  /**
   * Reset the current timer.
   * Clears and removes the current interval and resets time.
   *
   * @memberof Timer
   */
  reset() {
    clearTimeout(this.timerInt)
    delete this.timerInt
    this.time = 0
    EventBus.$emit('timer-reset')
  }

  /**
   * Resume the timer from a paused state.
   *
   * @memberof Timer
   */
  resume() {
    if (!this.timerInt) {
      this.start()
      EventBus.$emit('timer-resumed')
    }
  }
}
