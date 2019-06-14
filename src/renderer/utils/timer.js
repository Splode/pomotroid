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
    if (!this.timerHandle) {
      // stores the last time at least one second passed
      let previous = performance.now()

      /**
       * @param {number} now seconds since the page loaded (high precision)
       */
      const timerLoop = now => {
        // count seconds since the last time at leat one second passed
        const seconds = (now - previous) / 1000

        if (seconds >= 1) {
          const flooredSeconds = Math.floor(seconds)

          // overwrite the previous, taking into account the uncounted milliseconds
          const carryMillis = (seconds - flooredSeconds) * 1000
          previous = now - carryMillis

          // add seconds as integers
          this.time += flooredSeconds

          if (this.time >= this.totalSeconds) {
            this.pause()
            EventBus.$emit('timer-completed')
            // return to prevent the next animation frame
            return
          } else {
            // if the callback took more than a second, it is possible that multiple seconds are added at once
            // if that is the case, "timer-advanced" will only fire once
            // (it is possible to make a quick loop that will fire "timer-advanced" for each passed seconds if requires)
            EventBus.$emit('timer-advanced', this.time, this.totalSeconds)
          }
        }

        this.timerHandle = requestAnimationFrame(timerLoop)
      }

      // first call is always in 1000 ms
      this.timerHandle = requestAnimationFrame(timerLoop)
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
    cancelAnimationFrame(this.timerHandle)
    delete this.timerHandle
    EventBus.$emit('timer-paused')
  }

  /**
   * Reset the current timer.
   * Clears and removes the current interval and resets time.
   *
   * @memberof Timer
   */
  reset() {
    cancelAnimationFrame(this.timerHandle)
    delete this.timerHandle
    this.time = 0
    EventBus.$emit('timer-reset')
  }

  /**
   * Resume the timer from a paused state.
   *
   * @memberof Timer
   */
  resume() {
    if (!this.timerHandle) {
      this.start()
      EventBus.$emit('timer-resumed')
    }
  }
}
