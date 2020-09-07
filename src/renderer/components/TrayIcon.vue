<template> </template>

<script>
import { ipcRenderer } from 'electron'
import { EventBus } from '@/utils/EventBus'

export default {
  data() {
    return {
      currentRound: null,
      paused: true,
      lastElapsed: 0,
      total: 1
    }
  },

  computed: {
    minToTray() {
      return this.$store.getters.minToTray
    }
  },

  mounted() {
    const updateTrayImage = (elapsed, total) => {
      if (!this.minToTray) {
        return
      }
      if (
        !this.paused &&
        this.lastElapsed < elapsed &&
        (elapsed - this.lastElapsed) / total < 0.01
      ) {
        // avoid updates without visual difference
        return
      }
      this.lastElapsed = elapsed
      this.total = total
      const image = createTrayImage(
        this.currentRound,
        this.paused,
        this.lastElapsed,
        this.total
      )
      ipcRenderer.send('tray-icon-update', image)
    }

    EventBus.$on('ready-long-break', () => {
      this.currentRound = 'long-break'
      this.lastElapsed = -1
      updateTrayImage(0, 1)
    })

    EventBus.$on('ready-short-break', () => {
      this.currentRound = 'short-break'
      this.lastElapsed = -1
      updateTrayImage(0, 1)
    })

    EventBus.$on('ready-work', () => {
      this.currentRound = 'work'
      this.lastElapsed = -1
      updateTrayImage(0, 1)
    })

    EventBus.$on('timer-reset', () => {
      this.paused = true
      this.lastElapsed = -1
      updateTrayImage(0, 1)
    })

    EventBus.$on('timer-tick', payload => {
      updateTrayImage(payload.elapsed, payload.total)
    })

    EventBus.$on('timer-started', () => {
      this.paused = false
      updateTrayImage(this.lastElapsed, this.total)
    })

    EventBus.$on('timer-paused', () => {
      this.paused = true
      updateTrayImage(this.lastElapsed, this.total)
    })

    EventBus.$on('timer-completed', () => {
      this.paused = true
      updateTrayImage(this.lastElapsed, this.total)
    })

    updateTrayImage(this.lastElapsed, this.total)
  }
}

function createTrayImage(currentRound, paused, elapsed, total) {
  const bgVar = document.documentElement.style.getPropertyValue(
    '--color-background'
  )
  const focusVar = document.documentElement.style.getPropertyValue(
    '--color-focus-round'
  )
  const shortVar = document.documentElement.style.getPropertyValue(
    '--color-short-round'
  )
  const longVar = document.documentElement.style.getPropertyValue(
    '--color-long-round'
  )
  const size = setSizeTrayImage()
  const bgColor = !bgVar ? '#2F384B' : bgVar
  const workColor = !focusVar ? '#FF4E4D' : focusVar
  const shortBreakColor = !shortVar ? '#05EB8B' : shortVar
  const longBreakColor = !longVar ? '#0BBCDA' : longVar
  const arcRadiusRatio = 0.55
  const arcLineWidthRatio = 0.3

  const remainingTime = 1 - elapsed / total
  const fgColor =
    currentRound === 'short-break' ? shortBreakColor : currentRound === 'long-break'
      ? longBreakColor
      : workColor
  const outerRadius = size / 2
  const innerRadius = outerRadius * arcRadiusRatio
  const lineWidth = outerRadius * arcLineWidthRatio
  const fullCircle = 2 * Math.PI
  const startAngle = -Math.PI / 2
  const endAngle = remainingTime * fullCircle + startAngle
  const center = outerRadius
  const pauseStrokesHalfDistance = innerRadius / 1.7

  const canvas = document.createElement('canvas')
  canvas.width = size
  canvas.height = size

  const ctx = canvas.getContext('2d')

  ctx.fillStyle = bgColor
  ctx.strokeStyle = fgColor
  ctx.lineWidth = lineWidth

  ctx.beginPath()
  ctx.arc(center, center, outerRadius, 0, fullCircle, false)
  ctx.fill()

  if (paused) {
    ctx.beginPath()
    ctx.moveTo(center - pauseStrokesHalfDistance, center - innerRadius)
    ctx.lineTo(center - pauseStrokesHalfDistance, center + innerRadius)
    ctx.moveTo(center + pauseStrokesHalfDistance, center - innerRadius)
    ctx.lineTo(center + pauseStrokesHalfDistance, center + innerRadius)
    ctx.stroke()
  } else {
    ctx.beginPath()
    ctx.arc(center, center, innerRadius, startAngle, endAngle, false)
    ctx.stroke()
  }

  const dataUrl = canvas.toDataURL('image/png')
  return dataUrl
}

function setSizeTrayImage() {
  return process.platform === 'darwin' ? 19 : 32
}
</script>
