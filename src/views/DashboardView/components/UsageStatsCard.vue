<script setup lang="ts">
import { NCard, NSpace, NProgress, NNumberAnimation } from 'naive-ui'
import { computed } from 'vue'
import { useI18n } from '../../../locales'
import type { DeviceInfoState } from '../types'

const props = defineProps<{
  deviceInfo: DeviceInfoState
  loading: boolean
}>()

const { i18n } = useI18n()

// 计算使用量百分比
const getUsagePercentage = (used: number, total: number) => {
  if (!total) return 0
  return Math.min(100, Math.round((used / total) * 100))
}

// 普通账户使用量百分比
const accountUsagePercentage = computed(() => {
  if (!props.deviceInfo.userInfo?.totalCount) return 0
  return getUsagePercentage(
    props.deviceInfo.userInfo.usedCount,
    props.deviceInfo.userInfo.totalCount
  )
})

// Cursor高级模型使用量百分比
const cursorGpt4Percentage = computed(() => {
  const usage = props.deviceInfo.cursorInfo.usage?.['gpt-4']
  if (!usage) return 0
  return getUsagePercentage(usage.numRequests, usage.maxRequestUsage || 0)
})

// Cursor普通模型使用量百分比
const cursorGpt35Percentage = computed(() => {
  const usage = props.deviceInfo.cursorInfo.usage?.['gpt-3.5-turbo']
  if (!usage) return 0
  if (!usage.maxRequestUsage) return 100
  return getUsagePercentage(usage.numRequests, usage.maxRequestUsage)
})
</script>

<template>
  <n-card :title="i18n.dashboard.usageStats" style="height: 100%; user-select: none;">
    <n-space vertical :size="24" style="height: 100%; justify-content: space-around;">
      <!-- 账户使用统计 -->
      <n-space vertical :size="8" class="cursor-pool-usage">
        <n-space justify="space-between">
          <span>{{ i18n.dashboard.cpUsage }}</span>
          <n-space :size="0">
            <n-number-animation 
              :from="0" 
              :to="(deviceInfo.userInfo?.usedCount || 0) * 50"
              :duration="1000"
            />
            <span>/{{ (deviceInfo.userInfo?.totalCount || 0) * 50 }}</span>
          </n-space>
        </n-space>
        <n-progress
          type="line"
          status="success"
          :percentage="accountUsagePercentage"
          :show-indicator="false"
          :height="12"
          :border-radius="6"
          :processing="loading"
        />
      </n-space>

      <!-- Cursor GPT-4 使用统计 -->
      <n-space vertical :size="8" class="advanced-model-usage">
        <n-space justify="space-between">
          <span>{{ i18n.dashboard.advancedModelUsage }}</span>
          <n-space v-if="deviceInfo.cursorInfo.usage" :size="0">
            <n-number-animation 
              :from="0"
              :to="deviceInfo.cursorInfo.usage['gpt-4']?.numRequests || 0"
              :duration="1000"
            />
            <span>/{{ deviceInfo.cursorInfo.usage['gpt-4']?.maxRequestUsage || 0 }}</span>
          </n-space>
          <span v-else>{{ i18n.dashboard.cannotGetUsage }}</span>
        </n-space>
        <n-progress
          type="line"
          status="success"
          :percentage="cursorGpt4Percentage"
          :show-indicator="false"
          :height="12"
          :border-radius="6"
          :processing="loading"
        />
      </n-space>

      <!-- Cursor GPT-3.5 使用统计 -->
      <n-space vertical :size="8" class="basic-model-usage">
        <n-space justify="space-between">
          <span>{{ i18n.dashboard.basicModelUsage }}</span>
          <n-space v-if="deviceInfo.cursorInfo.usage" :size="0">
            <n-number-animation 
              :from="0" 
              :to="deviceInfo.cursorInfo.usage['gpt-3.5-turbo']?.numRequests || 0"
              :duration="1000"
            />
            <span v-if="deviceInfo.cursorInfo.usage['gpt-3.5-turbo']?.maxRequestUsage">
              /{{ deviceInfo.cursorInfo.usage['gpt-3.5-turbo']?.maxRequestUsage }}
            </span>
            <span v-else>/{{ i18n.dashboard.unlimited }}</span>
          </n-space>
          <span v-else>{{ i18n.dashboard.cannotGetUsage }}</span>
        </n-space>
        <n-progress
          type="line"
          status="success"
          :percentage="cursorGpt35Percentage"
          :show-indicator="false"
          :height="12"
          :border-radius="6"
          :processing="loading"
        />
      </n-space>
    </n-space>
  </n-card>
</template> 