<script setup lang="ts">
import { NModal, NSpace, NButton } from 'naive-ui'
import type { PendingForceKillAction } from '../types'

defineProps<{
  show: boolean
  pendingAction: PendingForceKillAction | null
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'force-kill'): void
}>()

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const handleForceKill = () => {
  emit('force-kill')
}
</script>

<template>
  <n-modal
    :show="show"
    preset="dialog"
    title="Cursor 正在运行"
    :closable="true"
    :mask-closable="false"
    @update:show="updateShow"
  >
    <template #default>
      检测到 Cursor 正在运行, 请保存尚未更改的项目再继续操作!
    </template>
    <template #action>
      <n-space justify="end">
        <n-button type="warning" @click="handleForceKill">
          我已保存, 强制关闭
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template> 