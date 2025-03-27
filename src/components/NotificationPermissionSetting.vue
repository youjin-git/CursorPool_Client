<script setup lang="ts">
  import { computed, onMounted } from 'vue'
  import { useNotificationStore } from '@/stores'
  import { NSpace, NButton, useMessage, NTag } from 'naive-ui'
  import { useI18n } from '../locales'

  const notificationStore = useNotificationStore()
  const message = useMessage()
  const { t } = useI18n()

  // 检查权限状态
  const hasPermission = computed(() => notificationStore.permissionGranted === true)
  const isRequesting = computed(() => notificationStore.isRequesting)

  // 请求权限
  const requestPermission = async () => {
    const granted = await notificationStore.requestNotificationPermission()
    if (granted) {
      message.success(t('systemControl.messages.permissionGranted'))

      // 发送一个测试通知
      await notificationStore.notify({
        title: t('notification.testTitle'),
        body: t('notification.testBody'),
      })
    } else {
      message.error(t('systemControl.messages.permissionDenied'))
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
      <small style="margin-right: 8px; color: var(--n-text-color-3)"
        >{{ t('systemControl.systemNotification') }}:</small
      >
      <n-tag :type="hasPermission ? 'success' : 'error'" size="small" round>
        {{ hasPermission ? t('systemControl.authorized') : t('systemControl.unauthorized') }}
      </n-tag>
    </span>
    <n-button
      type="info"
      :loading="isRequesting"
      :disabled="hasPermission"
      style="width: 120px"
      @click="requestPermission"
    >
      {{ t('systemControl.requestPermission') }}
    </n-button>
  </n-space>
</template>
