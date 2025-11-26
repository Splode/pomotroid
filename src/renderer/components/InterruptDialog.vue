<template>
  <transition name="fade">
    <div v-if="show" class="InterruptDialog-overlay" @click="closeDialog">
      <div class="InterruptDialog" @click.stop>
      <div class="InterruptDialog-header">
        <h3>番茄钟中断</h3>
      </div>

      <div class="InterruptDialog-content">
        <p class="InterruptDialog-message">请选择中断原因（可选）：</p>
                  <div class="InterruptDialog-reasons">
            <button
              v-for="reason in predefinedReasons"
              :key="reason"
              class="InterruptDialog-reason-btn"
              :class="{ 'is-selected': selectedReason === reason }"
              @click="selectReason(reason)"
            >
              {{ reason }}
            </button>
          </div>

          <div class="InterruptDialog-custom">
            <input
              v-model="customReason"
              type="text"
              class="InterruptDialog-input"
              placeholder="或输入自定义原因..."
              @focus="clearSelectedReason"
              maxlength="50"
            />
          </div>
        </div>

        <div class="InterruptDialog-actions">
          <button class="InterruptDialog-btn InterruptDialog-btn--cancel" @click="skipReason">
            跳过
          </button>
          <button class="InterruptDialog-btn InterruptDialog-btn--confirm" @click="submitReason">
            确定
          </button>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  name: 'InterruptDialog',

  data() {
    return {
      selectedReason: null,
      customReason: '',
      predefinedReasons: [
        '紧急事项',
        '会议',
        '电话',
        '休息',
        '其他人打断',
        '突发情况'
      ]
    }
  },

  computed: {
    show() {
      return this.$store.getters.showInterruptDialog
    }
  },

  methods: {
    selectReason(reason) {
      this.selectedReason = reason
      this.customReason = ''
    },

    clearSelectedReason() {
      this.selectedReason = null
    },

    submitReason() {
      const reason = this.customReason.trim() || this.selectedReason || '未指定原因'
      this.$store.dispatch('submitInterruptReason', reason)
      this.closeDialogOnly()
      this.resetForm()
    },

    skipReason() {
      this.$store.dispatch('submitInterruptReason', '未指定原因')
      this.closeDialogOnly()
      this.resetForm()
    },

    closeDialog() {
      // 点击背景关闭时，也要提交中断
      this.$store.dispatch('submitInterruptReason', '用户取消')
      this.closeDialogOnly()
      this.resetForm()
    },

    closeDialogOnly() {
      this.$store.dispatch('closeInterruptDialog')
    },

    resetForm() {
      this.selectedReason = null
      this.customReason = ''
    }
  }
}
</script>

<style lang="scss" scoped>
.InterruptDialog-overlay {
  align-items: center;
  background-color: rgba(0, 0, 0, 0.6);
  display: flex;
  justify-content: center;
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1000;
  -webkit-app-region: no-drag;
}

.InterruptDialog {
  background-color: var(--color-background-light);
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  max-width: 480px;
  width: 90%;
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.InterruptDialog-header {
  border-bottom: 1px solid var(--color-background);
  padding: 20px 24px;

  h3 {
    color: var(--color-foreground);
    font-size: 18px;
    font-weight: 600;
    margin: 0;
  }
}

.InterruptDialog-content {
  padding: 24px;
}

.InterruptDialog-message {
  color: var(--color-foreground-darker);
  font-size: 14px;
  margin: 0 0 16px 0;
}

.InterruptDialog-reasons {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
  margin-bottom: 16px;
}

.InterruptDialog-reason-btn {
  background-color: var(--color-background);
  border: 2px solid transparent;
  border-radius: 6px;
  color: var(--color-foreground-darker);
  cursor: pointer;
  font-size: 13px;
  padding: 10px 16px;
  transition: all 0.2s ease;

  &:hover {
    background-color: var(--color-background-lightest);
    border-color: var(--color-accent);
  }

  &.is-selected {
    background-color: var(--color-accent);
    border-color: var(--color-accent);
    color: var(--color-background);
    font-weight: 600;
  }
}

.InterruptDialog-custom {
  margin-top: 16px;
}

.InterruptDialog-input {
  background-color: var(--color-background);
  border: 2px solid var(--color-background);
  border-radius: 6px;
  color: var(--color-foreground);
  font-size: 14px;
  padding: 10px 12px;
  transition: border-color 0.2s ease;
  width: 100%;

  &:focus {
    border-color: var(--color-accent);
    outline: none;
  }

  &::placeholder {
    color: var(--color-foreground-darkest);
  }
}

.InterruptDialog-actions {
  border-top: 1px solid var(--color-background);
  display: flex;
  gap: 12px;
  padding: 16px 24px;
}

.InterruptDialog-btn {
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  flex: 1;
  padding: 10px 20px;
  transition: all 0.2s ease;

  &--cancel {
    background-color: var(--color-background);
    color: var(--color-foreground-darker);

    &:hover {
      background-color: var(--color-background-lightest);
    }
  }

  &--confirm {
    background-color: var(--color-accent);
    color: var(--color-background);

    &:hover {
      opacity: 0.9;
      transform: translateY(-1px);
    }
  }
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter,
.fade-leave-to {
  opacity: 0;
}
</style>
