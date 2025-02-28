<script setup lang="ts">
import { NCard, NSpace, NButton } from 'naive-ui'
import { useI18n } from '../../../locales'
import type { DeviceInfoState } from '../types'
import { useDashboardState } from '../composables/useDashboardState'

defineProps<{
  deviceInfo: DeviceInfoState
}>()

const { 
  machineCodeLoading,
  accountSwitchLoading,
  quickChangeLoading 
} = useDashboardState()

const emit = defineEmits<{
  (e: 'machine-code-change'): void
  (e: 'account-switch'): void
  (e: 'quick-change'): void
}>()

const { i18n } = useI18n()

const handleMachineCodeChange = () => {
  emit('machine-code-change')
}

const handleAccountSwitch = () => {
  emit('account-switch')
}

const handleQuickChange = () => {
  emit('quick-change')
}
</script>

<template>
  <n-card :title="i18n.dashboard.quickActions" class="quick-actions-card" style="user-select: none;">
    <n-space vertical>
      <n-space justify="space-around">
        <n-button type="primary" @click="handleQuickChange" :disabled="!deviceInfo.userInfo" :loading="quickChangeLoading">
          {{ i18n.dashboard.quickChange }}
        </n-button>
        <n-button type="primary" @click="handleAccountSwitch" :disabled="!deviceInfo.userInfo" :loading="accountSwitchLoading">
          {{ i18n.dashboard.changeAccount }}
        </n-button>
        <n-button type="primary" @click="handleMachineCodeChange" :loading="machineCodeLoading">
          {{ i18n.dashboard.changeMachineCode }}
        </n-button>
      </n-space>
    </n-space>
  </n-card>
</template> 