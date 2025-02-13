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

const message = useMessage()

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
    title: '邮箱',
    key: 'email'
  },
  {
    title: '状态',
    key: 'status'
  },
  {
    title: '最后使用时间',
    key: 'lastUsed'
  },
  {
    title: '操作',
    key: 'actions',
    render(row) {
      return (
        <NSpace>
          <NButton size="small" onClick={() => handleSwitch(row)}>
            切换到此账户
          </NButton>
          <NButton size="small" type="error" onClick={() => handleDelete(row)}>
            删除
          </NButton>
        </NSpace>
      )
    }
  }
]

const handleSwitch = (account: Account) => {
  message.success(`切换到账户: ${account.email}`)
}

const handleDelete = (account: Account) => {
  accounts.value = accounts.value.filter(a => a.id !== account.id)
  message.success('删除成功')
}

const newAccount = ref('')

const handleAdd = () => {
  if (!newAccount.value) {
    message.warning('请输入账户邮箱')
    return
  }
  
  accounts.value.push({
    id: Date.now(),
    email: newAccount.value,
    status: 'inactive',
    lastUsed: '-'
  })
  
  newAccount.value = ''
  message.success('添加成功')
}
</script>

<template>
  <n-space vertical :size="24">
    <n-card title="添加账户">
      <n-space>
        <n-input v-model:value="newAccount" placeholder="请输入账户邮箱" />
        <n-button type="primary" @click="handleAdd">
          添加
        </n-button>
      </n-space>
    </n-card>
    
    <n-card title="账户列表">
      <n-data-table
        :columns="columns"
        :data="accounts"
        :bordered="false"
      />
    </n-card>
  </n-space>
</template> 