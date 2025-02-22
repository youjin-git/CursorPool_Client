<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { NCard, NDataTable, NSpace, NDatePicker, NButton, useMessage } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import type { OperationRecord } from '../types/history'

const { currentLang, i18n } = useI18n()
const message = useMessage()

const records = ref<OperationRecord[]>([])
const dateRange = ref<[number, number] | null>(null)

// 加载历史记录
const loadHistory = () => {
  const history = JSON.parse(localStorage.getItem('operation_history') || '[]')
  records.value = history
}

// 清除历史记录
const clearHistory = () => {
  try {
    localStorage.setItem('operation_history', '[]')
    records.value = []
    message.success(i18n.value.history.clearSuccess)
    // 触发历史记录更新事件
    window.dispatchEvent(new Event('history_updated'))
  } catch (error) {
    message.error(i18n.value.history.clearFailed)
  }
}

// 监听历史记录更新
const handleHistoryUpdate = () => {
  loadHistory()
}

onMounted(() => {
  loadHistory()
  window.addEventListener('history_updated', handleHistoryUpdate)
})

onUnmounted(() => {
  window.removeEventListener('history_updated', handleHistoryUpdate)
})

const columns: DataTableColumns<OperationRecord> = [
  {
    title: () => messages[currentLang.value].history.type,
    key: 'type'
  },
  {
    title: () => messages[currentLang.value].history.detail,
    key: 'detail'
  },
  {
    title: () => messages[currentLang.value].history.time,
    key: 'timestamp',
    sorter: 'default'
  },
  {
    title: () => messages[currentLang.value].history.operator,
    key: 'operator'
  }
]
</script>

<template>
  <n-space vertical :size="24">
    <n-card :title="messages[currentLang].history.filter">
      <n-space justify="space-between">
        <n-date-picker
          v-model:value="dateRange"
          type="daterange"
          clearable
          :placeholder="i18n.history.datePlaceholder"
        />
        <n-button 
          type="error" 
          @click="clearHistory"
          :disabled="records.length === 0"
        >
          {{ i18n.history.clearHistory }}
        </n-button>
      </n-space>
    </n-card>

    <n-card :title="messages[currentLang].history.title">
      <n-data-table
        :columns="columns"
        :data="records"
        :bordered="false"
        :pagination="{
          pageSize: 10
        }"
      />
    </n-card>
  </n-space>
</template> 