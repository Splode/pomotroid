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
      this.timerInt = setInterval(() => {
        this.time += 1
        if (this.time >= this.totalSeconds) {
          this.pause()
          EventBus.$emit('timer-completed')
        } else {
          EventBus.$emit('timer-advanced', this.time, this.totalSeconds)
        }
      }, 1000)
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
    clearInterval(this.timerInt)
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
    clearInterval(this.timerInt)
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
