<script setup lang="ts">
  import { computed, onMounted } from 'vue'
  import { useNotificationStore } from '@/stores'
  import { NSpace, NButton, useMessage } from 'naive-ui'

  const notificationStore = useNotificationStore()
  const message = useMessage()

  // 检查权限状态
  const hasPermission = computed(() => notificationStore.permissionGranted === true)
  const isRequesting = computed(() => notificationStore.isRequesting)

  // 请求权限
  const requestPermission = async () => {
    const granted = await notificationStore.requestNotificationPermission()
    if (granted) {
      message.success('通知权限已授予')

      // 发送一个测试通知
      await notificationStore.notify({
        title: '通知测试',
        body: '恭喜！通知功能已成功启用。',
      })
    } else {
      message.error('通知权限被拒绝，请在系统设置中手动启用')
    }
  }

  // 组件加载时检查权限
  onMounted(async () => {
    await notificationStore.checkPermission()
  })
</script>

<template>
  <n-space justify="space-between" align="center">
    <span>
      <small style="margin-right: 8px; color: var(--n-text-color-3)">系统通知:</small>
      {{ hasPermission ? '已授权' : '未授权' }}
    </span>
    <n-button
      type="primary"
      :loading="isRequesting"
      :disabled="hasPermission"
      style="width: 120px"
      @click="requestPermission"
    >
      请求权限
    </n-button>
  </n-space>
</template>
