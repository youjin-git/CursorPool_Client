<script setup lang="ts">
import { ref } from 'vue'
import { NCard, NDataTable, NSpace, NDatePicker } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'

const { currentLang } = useI18n()

interface OperationRecord {
  id: number
  type: string
  detail: string
  timestamp: string
  operator: string
}

const records = ref<OperationRecord[]>([
  {
    id: 1,
    type: '账户切换',
    detail: '切换到账户: test@example.com',
    timestamp: '2024-03-20 12:00:00',
    operator: 'System'
  },
  {
    id: 2,
    type: '机器码修改',
    detail: '修改机器码: XXXX-XXXX-XXXX-XXXX',
    timestamp: '2024-03-20 12:01:00',
    operator: 'System'
  }
])

const dateRange = ref<[number, number] | null>(null)

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
      <n-space>
        <n-date-picker
          v-model:value="dateRange"
          type="daterange"
          clearable
          :placeholder="messages[currentLang].history.dateRange"
        />
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