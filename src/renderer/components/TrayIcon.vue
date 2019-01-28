<template>
</template>

<script>
import { ipcRenderer } from 'electron'
import { EventBus } from '@/utils/event-bus'

export default {
  data() {
    return {
      state: null,
      lastUpdate: 0
    }
  },

  computed: {
    minToTray() {
      return this.$store.getters.minToTray
    }
  },

  mounted() {
    let updateTrayImage = (elapsed, total) => {
      if (!this.minToTray) {
        return
      }
      if (
        this.lastUpdate < elapsed &&
        (elapsed - this.lastUpdate) / total < 0.01
      ) {
        // avoid updates without visual difference
        return
      }
      this.lastUpdate = elapsed
      const image = createTrayImage(this.state, elapsed, total)
      ipcRenderer.send('tray-icon-update', image)
    }

    EventBus.$on('ready-long-break', () => {
      this.state = 'long-break'
      updateTrayImage(0, 1)
    })

    EventBus.$on('ready-short-break', () => {
      this.state = 'short-break'
      updateTrayImage(0, 1)
    })

    EventBus.$on('ready-work', () => {
      this.state = 'work'
      updateTrayImage(0, 1)
    })

    EventBus.$on('timer-advanced', (elapsed, total) => {
      updateTrayImage(elapsed, total)
    })
  }
}

function createTrayImage(state, elapsed, total) {
  const size = 32
  const bgColor = '#2F384B'
  const workColor = '#FF4E4D'
  const shortBreakColor = '#05EB8B'
  const longBreakColor = '#0BBCDA'
  const arcRadiusRatio = 0.55
  const arcLineWidthRatio = 0.3

  const remainingTime = 1 - elapsed / total
  const arcColor =
    state === 'short-break'
      ? shortBreakColor
      : state === 'long-break'
        ? longBreakColor
        : workColor
  const outerRadius = size / 2
  const innerRadius = outerRadius * arcRadiusRatio
  const lineWidth = outerRadius * arcLineWidthRatio
  const fullCircle = 2 * Math.PI
  const startAngle = -Math.PI / 2
  const endAngle = remainingTime * fullCircle + startAngle
  const center = outerRadius

  const canvas = document.createElement('canvas')
  canvas.width = size
  canvas.height = size

  const ctx = canvas.getContext('2d')

  ctx.fillStyle = bgColor
  ctx.strokeStyle = arcColor
  ctx.lineWidth = lineWidth

  ctx.beginPath()
  ctx.arc(center, center, outerRadius, 0, fullCircle, false)
  ctx.fill()

  ctx.beginPath()
  ctx.arc(center, center, innerRadius, startAngle, endAngle, false)
  ctx.stroke()

  const dataUrl = canvas.toDataURL('image/png')
  return dataUrl
}
</script>
