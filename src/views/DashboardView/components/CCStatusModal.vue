<script setup lang="ts">
import { NModal, NSpace, NButton } from 'naive-ui'
import { useHookActions } from '../composables/useHookActions'

defineProps<{
  show: boolean
  originalAction: { type: 'machine' | 'account' | 'quick' | null }
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
}>()

const { applyHookLoading, handleApplyHook } = useHookActions()

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const handleApplyHookClick = async () => {
  await handleApplyHook()
  updateShow(false)
}
</script>

<template>
  <n-modal
    :show="show"
    preset="dialog"
    title="CC 客户端未注入"
    :closable="true"
    :mask-closable="true"
    @update:show="updateShow"
  >
    <template #default>
      <p>检测到 Cursor 客户端未注入，在此状态下进行操作可能会导致：</p>
      <ul style="margin: 12px 0; padding-left: 20px; color: #ff4d4f;">
        <li>账户更换失败</li>
        <li>积分异常扣除</li>
      </ul>
      <p>请确保 Cursor 客户端正常注入后再进行操作</p>
    </template>
    <template #action>
      <n-space justify="end">
        <n-button tertiary @click="updateShow(false)" style="margin-right: auto">
          我知道了
        </n-button>
        <n-button type="primary" @click="handleApplyHookClick" :loading="applyHookLoading">
          自动注入
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template> 