<script setup lang="ts">
import { ref, onMounted, h } from 'vue'
import { useMessage } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { NDataTable, NSpace, NButton, NCard, NModal } from 'naive-ui'
import type { HistoryAccount } from '@/types/history'
import { getHistoryAccounts, removeHistoryAccount } from '@/utils/historyAccounts'
import { getUsage, switchAccount, resetMachineId, checkCursorRunning, checkHookStatus, applyHook, closeCursor, launchCursor } from '@/api'
import type { PendingForceKillAction } from '@/types/dashboard'

const message = useMessage()
const loading = ref(false)
const switchLoadingMap = ref<Record<string, boolean>>({})
const deleteLoadingMap = ref<Record<string, boolean>>({})
const accounts = ref<HistoryAccount[]>([])

// 添加模态框相关状态
const showCursorRunningModal = ref(false)
const pendingAccount = ref<HistoryAccount | null>(null)

const autoApplyHook = async (): Promise<boolean> => {
  try {
    message.loading('正在自动注入...', { duration: 0 })
    await applyHook(false)
    message.destroyAll()
    message.success('注入成功')
    
    const hookStatus = await checkHookStatus()
    return hookStatus === true
  } catch (error) {
    console.error('自动注入失败:', error)
    message.destroyAll()
    message.error(error instanceof Error ? error.message : '注入失败，请手动注入后再试')
    return false
  }
}

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
      // 显示自己的模态框
      pendingAccount.value = account
      showCursorRunningModal.value = true
      return
    }

    const hookStatus = await checkHookStatus()
    if (!hookStatus) {
      const hookSuccess = await autoApplyHook()
      if (!hookSuccess) {
        return
      }
    }

    await resetMachineId({ machineId: account.machineCode })
    await switchAccount(account.email, account.token, false)
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.success('切换账户成功')
    window.location.reload()
  } catch (error) {
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

const handleForceKill = async () => {
  if (!pendingAccount.value) return
  
  try {
    showCursorRunningModal.value = false
    const account = pendingAccount.value
    switchLoadingMap.value[account.email] = true
    
    message.loading('正在关闭 Cursor...', { duration: 0 })
    await closeCursor()
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.destroyAll()
    
    // 检查是否需要注入
    if (!(await checkHookStatus())) {
      message.loading('正在注入...', { duration: 0 })
      const hookSuccess = await autoApplyHook()
      if (!hookSuccess) {
        message.error('注入失败，请手动注入后再试')
        return
      }
      message.destroyAll()
    }
    
    // 执行账户切换
    message.loading('正在切换账户...', { duration: 0 })
    await resetMachineId({ machineId: account.machineCode })
    await switchAccount(account.email, account.token, false)
    await new Promise(resolve => setTimeout(resolve, 1000))
    message.destroyAll()
    message.success('切换账户成功')
    
    // 直接启动Cursor
    message.loading('正在启动 Cursor...', { duration: 0 })
    try {
      await launchCursor()
      message.destroyAll()
      message.success('Cursor 已启动')
    } catch (launchError) {
      message.destroyAll()
      message.error('启动 Cursor 失败: ' + (launchError instanceof Error ? launchError.message : String(launchError)))
    }
    
    window.location.reload()
  } catch (error) {
    message.destroyAll()
    message.error('切换账户失败')
  } finally {
    switchLoadingMap.value[pendingAccount.value.email] = false
    pendingAccount.value = null
  }
}

onMounted(() => {
  accounts.value = getHistoryAccounts()
  window.addEventListener('force_kill_cursor', async (e: Event) => {
    const detail = (e as CustomEvent).detail as PendingForceKillAction
    if (detail.type === 'account') {
      await handleForceKill()
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

  <!-- 修改 Cursor 运行提示模态框 -->
  <n-modal
    v-model:show="showCursorRunningModal"
    preset="dialog"
    title="Cursor 正在运行"
    :closable="true"
    :mask-closable="false"
  >
    <template #default>
      检测到 Cursor 正在运行, 请保存尚未更改的项目再继续操作!
    </template>
    <template #action>
      <n-space justify="end">
        <n-button type="warning" @click="handleForceKill">
          我已保存, 强制关闭
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template> 