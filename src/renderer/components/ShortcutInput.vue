<template>
  <div>
    <input
      @keyup.stop.prevent="keyup"
      type="text"
      :value="value"
      @keypress="() => false"
      spellcheck="false"
    />
  </div>
</template>

<script>
export default {
  name: 'Drawer-settings',
  props: ['value'],
  methods: {
    keyup(event) {
      const keyCode = event.keyCode
      const key = event.key
      // const charCode = event.code

      if ((keyCode >= 16 && keyCode <= 18) || keyCode === 91) return

      const result = []
      if (event.ctrlKey) {
        result.push('Control')
      }
      if (event.metaKey) {
        result.push('Super')
      } // this doesnt work I dont know why
      if (event.shiftKey) {
        result.push('Shift')
      }
      if (event.altKey) {
        result.push('Alt')
      }
      result.push(key.toUpperCase())

      this.$emit('input', result.join('+'))

      if (event.preventDefault) event.preventDefault()
      else event.returnValue = false
      if (event.stopPropagation) event.stopPropagation()
      if (event.cancelBubble) event.cancelBubble = true

      document.activeElement.blur()

      return false
    }
  }
}
</script>

<style lang="scss" scoped>
input {
  background: var(--color-background-light);
  border: 2px solid transparent;
  border-radius: 0.2rem;
  color: var(--color-accent);
  float: right;
  font-family: monospace;
  font-size: 1rem;
  max-width: 50%;
  text-align: center;
}

input:focus {
  background: var(--color-background);
  border: 2px solid var(--color-background-lightest);
  color: var(--color-accent);
  outline: 0;
}
</style>
