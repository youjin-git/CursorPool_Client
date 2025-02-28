<script setup lang="ts">
import { ref, onMounted, h, inject } from 'vue'
import { useMessage } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { NDataTable, NSpace, NButton, NCard } from 'naive-ui'
import type { HistoryAccount } from '@/types/history'
import { getHistoryAccounts, removeHistoryAccount } from '@/utils/historyAccounts'
import { getUsage, switchAccount, resetMachineId, checkCursorRunning } from '@/api'
import type { PendingForceKillAction } from '../views/DashboardView/types'

const message = useMessage()
const loading = ref(false)
const switchLoadingMap = ref<Record<string, boolean>>({})
const deleteLoadingMap = ref<Record<string, boolean>>({})
const accounts = ref<HistoryAccount[]>([])
const showCursorModal = inject<(action: PendingForceKillAction) => void>('showCursorModal')

const columns: DataTableColumns<HistoryAccount> = [
  {
    title: '邮箱',
    key: 'email'
  },
  {
    title: '机器码',
    key: 'machineCode',
    render(row) {
      const code = row.machineCode
      return code.slice(0, 4) + '...' + code.slice(-4)
    }
  },
  {
    title: 'GPT-4 使用次数',
    key: 'gpt4Count'
  },
  {
    title: 'GPT-3.5 使用次数',
    key: 'gpt35Count'
  },
  {
    title: '最后使用时间',
    key: 'lastUsed',
    render(row) {
      return new Date(row.lastUsed).toLocaleString()
    }
  },
  {
    title: '操作',
    key: 'actions',
    render(row) {
      return h(NSpace, null, {
        default: () => [
          h(
            NButton,
            {
              size: 'small',
              loading: switchLoadingMap.value[row.email] || false,
              onClick: () => handleSwitch(row)
            },
            { default: () => '切换到此账户' }
          ),
          h(
            NButton,
            {
              size: 'small',
              type: 'error',
              loading: deleteLoadingMap.value[row.email] || false,
              onClick: () => handleRemove(row)
            },
            { default: () => '删除' }
          )
        ]
      })
    }
  }
]

async function refreshUsage() {
  loading.value = true
  try {
    const history = getHistoryAccounts()
    for (const account of history) {
      try {
        const usage = await getUsage(account.token)
        account.gpt4Count = usage['gpt-4']?.numRequests || 0
        account.gpt35Count = usage['gpt-3.5-turbo']?.numRequests || 0
        account.lastUsed = Date.now()
      } catch (error) {
        console.error(`获取账户 ${account.email} 使用情况失败:`, error)
      }
    }
    accounts.value = history
  } catch (error) {
    message.error('刷新使用情况失败')
  } finally {
    loading.value = false
  }
}

async function handleSwitch(account: HistoryAccount) {
  try {
    switchLoadingMap.value[account.email] = true
    const isRunning = await checkCursorRunning()
    if (isRunning) {
      showCursorModal?.({
        type: 'account',
        params: account
      })
      return
    }

    await resetMachineId({ machineId: account.machineCode })
    await switchAccount(account.email, account.token, false)
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.success('切换账户成功')
    window.location.reload()
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    if (errorMsg === 'Cursor进程正在运行, 请先关闭Cursor') {
      showCursorModal?.({
        type: 'account',
        params: account
      })
      return
    }
    message.error('切换账户失败')
  } finally {
    switchLoadingMap.value[account.email] = false
  }
}

function handleRemove(account: HistoryAccount) {
  deleteLoadingMap.value[account.email] = true
  removeHistoryAccount(account.email)
  accounts.value = accounts.value.filter(a => a.email !== account.email)
  message.success('删除成功')
  deleteLoadingMap.value[account.email] = false
}

const handleForceKill = async (account: HistoryAccount) => {
  try {
    switchLoadingMap.value[account.email] = true
    await resetMachineId({ 
      machineId: account.machineCode,
      forceKill: true 
    })
    await switchAccount(
      account.email, 
      account.token,
      true
    )
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.success('切换账户成功')
    window.location.reload()
  } catch (error) {
    message.error('切换账户失败')
  } finally {
    switchLoadingMap.value[account.email] = false
  }
}

onMounted(() => {
  accounts.value = getHistoryAccounts()
  window.addEventListener('force_kill_cursor', async (e: Event) => {
    const detail = (e as CustomEvent).detail as PendingForceKillAction
    if (detail.type === 'account') {
      const account = detail.params as HistoryAccount
      await handleForceKill(account)
    }
  })
})
</script>

<template>
  <n-card title="历史账户">
    <n-space vertical>
      <n-space>
        <n-button 
          type="primary"
          :loading="loading"
          @click="refreshUsage"
        >
          刷新使用情况
        </n-button>
      </n-space>
      
      <n-data-table
        :columns="columns"
        :data="accounts"
        :loading="loading"
      />
    </n-space>
  </n-card>
</template> 