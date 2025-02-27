<script setup lang="ts">
import { NModal, NSpace, NButton } from 'naive-ui'

defineProps<{
  show: boolean
  unusedCredits: number
  pendingAction: 'account' | 'quick' | null
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'confirm'): void
  (e: 'cancel'): void
}>()

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const handleConfirm = () => {
  emit('confirm')
}

const handleCancel = () => {
  emit('cancel')
  updateShow(false)
}
</script>

<template>
  <n-modal
    :show="show"
    preset="dialog"
    title="使用提醒"
    :closable="true"
    :mask-closable="false"
    @update:show="updateShow"
  >
    <template #default>
      <p>您还有 {{ unusedCredits }} 次高级模型使用次数未使用</p>
      <p style="margin-top: 12px; color: #666;">
        {{ pendingAction === 'quick' ? '一键切换将扣除50积分' : '切换账号将扣除50积分' }}，确定要继续吗？
      </p>
    </template>
    <template #action>
      <n-space justify="end">
        <n-button @click="handleCancel">
          取消
        </n-button>
        <n-button type="primary" @click="handleConfirm">
          确认切换
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template> 