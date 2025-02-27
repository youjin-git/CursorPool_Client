<script setup lang="ts">
import { NModal, NSpace, NButton, NIcon } from 'naive-ui'
import { WarningOutlined } from '@vicons/antd'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'exit'): void
}>()

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const handleExit = () => {
  emit('exit')
}
</script>

<template>
  <n-modal
    :show="show"
    preset="dialog"
    title="需要管理员权限"
    :closable="false"
    :mask-closable="false"
    style="width: 500px"
    @update:show="updateShow"
  >
    <template #header>
      <n-space align="center">
        <n-icon size="24" color="#f0a020">
          <warning-outlined />
        </n-icon>
        <span>需要管理员权限</span>
      </n-space>
    </template>
    <div style="margin: 24px 0;">
      <p>本程序需要管理员权限才能正常运行。</p>
      <p style="margin-top: 12px; color: #999;">
        请右键点击程序图标,选择"以管理员身份运行"后重新启动程序。
      </p>
    </div>
    <template #action>
      <n-button type="error" @click="handleExit" block>
        退出程序
      </n-button>
    </template>
  </n-modal>
</template> 