export default class Timer extends EventTarget {
  constructor(min) {
    super()
    this.time = 0
    this.totalSeconds = min * 60

    this._complete = new Event('complete')
    this._pause = new Event('pause')
    this._reset = new Event('reset')
  }

  start() {
    if (!this.timerInt) {
      this.timerInt = setInterval(() => {
        this.time += 1
        if (this.time >= this.totalSeconds) {
          this.pause()
          this.dispatchEvent(this._complete)
        } else {
          this.dispatchEvent(
            new CustomEvent('tick', {
              detail: { time: this.time, totalSeconds: this.totalSeconds }
            })
          )
        }
      }, 1000)
      this.dispatchEvent(
        new CustomEvent('start', {
          detail: { time: this.time, totalSeconds: this.totalSeconds }
        })
      )
    }
  }

  pause() {
    clearInterval(this.timerInt)
    delete this.timerInt
    this.dispatchEvent(this._pause)
  }

  reset() {
    clearInterval(this.timerInt)
    delete this.timerInt
    this.time = 0
    this.dispatchEvent(this._reset)
  }

  resume() {
    if (!this.timerInt) {
      this.start()
      this.dispatchEvent(
        new CustomEvent('resume', {
          detail: { time: this.time, totalSeconds: this.totalSeconds }
        })
      )
    }
  }
}
