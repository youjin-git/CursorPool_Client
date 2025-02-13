<script setup lang="ts">
import { ref } from 'vue'
import { 
  NCard, 
  NSpace, 
  NDataTable, 
  NButton,
  NInput,
  useMessage
} from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { h } from 'vue'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'

const message = useMessage()
const { currentLang } = useI18n()

interface Account {
  id: number
  email: string
  status: 'active' | 'inactive'
  lastUsed: string
}

const accounts = ref<Account[]>([
  {
    id: 1,
    email: 'test@example.com',
    status: 'active',
    lastUsed: '2024-03-20 12:00:00'
  }
])

const columns: DataTableColumns<Account> = [
  {
    title: () => messages[currentLang.value].account.email,
    key: 'email'
  },
  {
    title: () => messages[currentLang.value].account.status,
    key: 'status',
    render: (row) => messages[currentLang.value].account[row.status]
  },
  {
    title: () => messages[currentLang.value].account.lastUsed,
    key: 'lastUsed'
  },
  {
    title: () => messages[currentLang.value].account.actions,
    key: 'actions',
    render: (row) => {
      return h(NSpace, null, {
        default: () => [
          h(
            NButton,
            {
              size: 'small',
              onClick: () => handleSwitch(row)
            },
            { default: () => messages[currentLang.value].account.switchTo }
          ),
          h(
            NButton,
            {
              size: 'small',
              type: 'error',
              onClick: () => handleDelete(row)
            },
            { default: () => messages[currentLang.value].account.delete }
          )
        ]
      })
    }
  }
]

const handleSwitch = (account: Account) => {
  message.success(
    messages[currentLang.value].message.switchSuccess.replace('{email}', account.email)
  )
}

const handleDelete = (account: Account) => {
  accounts.value = accounts.value.filter(a => a.id !== account.id)
  message.success(messages[currentLang.value].message.deleteSuccess)
}

const newAccount = ref('')

const handleAdd = () => {
  if (!newAccount.value) {
    message.warning(messages[currentLang.value].message.pleaseInputEmail)
    return
  }
  
  accounts.value.push({
    id: Date.now(),
    email: newAccount.value,
    status: 'inactive',
    lastUsed: '-'
  })
  
  newAccount.value = ''
  message.success(messages[currentLang.value].message.addSuccess)
}
</script>

<template>
  <n-space vertical :size="24">
    <n-card :title="messages[currentLang].account.addAccount">
      <n-space>
        <n-input
          v-model:value="newAccount"
          :placeholder="messages[currentLang].account.inputEmail"
        />
        <n-button type="primary" @click="handleAdd">
          {{ messages[currentLang].account.add }}
        </n-button>
      </n-space>
    </n-card>
    
    <n-card :title="messages[currentLang].account.accountList">
      <n-data-table
        :columns="columns"
        :data="accounts"
        :bordered="false"
      />
    </n-card>
  </n-space>
</template> 