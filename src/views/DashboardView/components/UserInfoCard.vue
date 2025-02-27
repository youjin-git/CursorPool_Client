<script setup lang="ts">
import { NCard, NSpace, NTag, NDivider, useMessage } from 'naive-ui'
import { computed } from 'vue'
import { useI18n } from '../../../locales'
import type { DeviceInfoState } from '../types'

defineProps<{
  deviceInfo: DeviceInfoState
  loading: boolean
}>()

const message = useMessage()
const { i18n } = useI18n()

// 格式化日期
const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
}

// 会员等级映射
type LevelMapType = {
  [key: number]: { 
    name: string; 
    type: 'default' | 'info' | 'success' | 'warning' | 'error' 
  }
}

const levelMap = computed<LevelMapType>(() => ({
  1: { name: i18n.value.dashboard.memberLevel[1], type: 'default' },
  2: { name: i18n.value.dashboard.memberLevel[2], type: 'info' },
  3: { name: i18n.value.dashboard.memberLevel[3], type: 'success' },
  4: { name: i18n.value.dashboard.memberLevel[4], type: 'warning' },
  5: { name: i18n.value.dashboard.memberLevel[5], type: 'error' }
}))

const copyText = (text: string) => {
  if (!text) return
  navigator.clipboard.writeText(text).then(() => {
    message.success(i18n.value.common.copySuccess)
  }).catch(() => {
    message.error(i18n.value.common.copyFailed)
  })
}
</script>

<template>
  <n-card :title="i18n.dashboard.userInfo" class="user-info-card" style="height: 100%; user-select: none;">
    <n-space vertical>
      <n-space vertical :size="12" style="user-select: none;">
        <n-space :size="8" style="line-height: 1.2;" class="user-info-username">
          <span style="width: 70px">{{ i18n.dashboard.username }}</span>
          <n-space :size="4" align="center">
            <span 
              style="font-size: 14px; cursor: pointer;" 
              @click="deviceInfo.userInfo?.username && copyText(deviceInfo.userInfo.username)"
            >{{ deviceInfo.userInfo?.username }}</span>
            <n-tag :type="levelMap[deviceInfo.userInfo?.level || 1].type" size="tiny" style="transform: scale(0.9)">
              {{ levelMap[deviceInfo.userInfo?.level || 1].name }}
            </n-tag>
          </n-space>
        </n-space>

        <n-divider style="margin: 0" />

        <n-space :size="8" style="line-height: 1.2;" class="user-info-email">
          <span style="width: 70px">{{ i18n.dashboard.email }}</span>
          <n-space :size="4" align="center">
            <span 
              style="font-size: 14px; cursor: pointer;" 
              @click="deviceInfo.cursorInfo.userInfo?.email && copyText(deviceInfo.cursorInfo.userInfo?.email)"
            >{{ deviceInfo.cursorInfo.userInfo?.email || '未绑定' }}</span>
          </n-space>
        </n-space>
        <n-space :size="8" style="line-height: 1.2;" class="user-info-cc-status">
          <span style="width: 70px">{{ i18n.dashboard.ccStatus }}</span>
          <n-tag :type="deviceInfo.hookStatus === true ? 'success' : 'error'" size="small">
            {{ deviceInfo.hookStatus === true ? i18n.systemControl.hookApplied : i18n.systemControl.hookNotApplied }}
          </n-tag>
        </n-space>
        <n-space :size="8" style="line-height: 1.2;" class="user-info-register-time">
          <span style="width: 70px">{{ i18n.dashboard.registerTime }}</span>
          <span 
            style="font-size: 14px; cursor: pointer;" 
            @click="copyText(deviceInfo.cursorInfo.usage?.startOfMonth ? formatDate(deviceInfo.cursorInfo.usage.startOfMonth) : '')"
          >{{ deviceInfo.cursorInfo.usage?.startOfMonth ? formatDate(deviceInfo.cursorInfo.usage.startOfMonth) : '未知' }}</span>
        </n-space>
        <span 
          style="font-size: 12px; color: #999; word-break: break-all; text-align: center; cursor: pointer;" 
          @click="copyText(deviceInfo.machineCode)"
          class="user-info-machine-code"
        >{{ deviceInfo.machineCode }}</span>
      </n-space>
    </n-space>
  </n-card>
</template> 