<script setup lang="ts">
import { ref, onMounted, h, computed } from 'vue'
import { useMessage } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'
import { NDataTable, NSpace, NButton, NCard, NModal, NProgress } from 'naive-ui'
import type { HistoryAccount } from '@/types/history'
import { getHistoryAccounts, removeHistoryAccount, saveAccountToHistory, syncLocalAccountsToBackend } from '@/utils/historyAccounts'
import { getUsage, switchAccount, resetMachineId, checkCursorRunning, checkHookStatus, applyHook, closeCursor, launchCursor, getMachineIds, clearHistoryAccounts } from '@/api'
import type { PendingForceKillAction } from '@/types/dashboard'

const message = useMessage()
const loading = ref(false)
const switchLoadingMap = ref<Record<string, boolean>>({})
const deleteLoadingMap = ref<Record<string, boolean>>({})
const accounts = ref<HistoryAccount[]>([])
const clearLoading = ref(false)

// 添加模态框相关状态
const showCursorRunningModal = ref(false)
const pendingAccount = ref<HistoryAccount | null>(null)

// 计算高使用量账户
const highUsageAccounts = computed(() => {
  return accounts.value.filter(account => {
    // 计算GPT-4使用率，如果超过90%则认为是高使用量账户
    const gpt4MaxUsage = account.gpt4MaxUsage || 500; // 如果没有最大使用量，默认为500
    const gpt4Usage = account.gpt4Count / gpt4MaxUsage * 100;
    return gpt4Usage >= 90;
  });
});

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

// 计算使用率
const calculateUsagePercent = (count: number, maxUsage: number | null | undefined) => {
  if (!maxUsage || maxUsage <= 0) {
    // 如果没有最大使用量或最大使用量为0，使用默认值
    maxUsage = count > 500 ? count : 500; // 如果当前使用量超过默认值，则使用当前使用量作为基准
  }
  const percent = (count / maxUsage) * 100;
  return Math.round(Math.min(percent, 100)); // 四舍五入并确保不超过100%
};

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
      return code.slice(0, 4) + '**' + code.slice(-4)
    }
  },
  {
    title: 'GPT-4使用量',
    key: 'gpt4Count',
    width: 150,
    render(row) {
      const percent = calculateUsagePercent(row.gpt4Count, row.gpt4MaxUsage);
      const color = percent >= 90 ? 'error' : percent >= 70 ? 'warning' : 'success';
      
      return h(NProgress, { 
        type: 'line', 
        percentage: percent, 
        status: color,
        indicatorPlacement: 'inside',
        height: 12,
        showIndicator: false
      });
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
            { default: () => '切换' }
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
    // 从后端获取历史账户
    const historyAccounts = await getHistoryAccounts()
    
    // 并发更新使用情况
    const updatePromises = historyAccounts.map(async (account) => {
      try {
        const usage = await getUsage(account.token)
        
        // 更新使用量和最大使用量
        account.gpt4Count = usage['gpt-4']?.numRequests || 0
        account.gpt35Count = usage['gpt-3.5-turbo']?.numRequests || 0
        account.gpt4MaxUsage = usage['gpt-4']?.maxRequestUsage || null
        account.gpt35MaxUsage = usage['gpt-3.5-turbo']?.maxRequestUsage || null
        account.lastUsed = Date.now()
        
        // 保存更新后的账户信息
        await saveAccountToHistory(account)
        return true;
      } catch (error) {
        console.error(`获取账户 ${account.email} 使用情况失败:`, error)
        return false;
      }
    })
    
    // 等待所有请求完成
    const results = await Promise.all(updatePromises)
    const successCount = results.filter(Boolean).length
    
    // 重新获取更新后的账户列表
    accounts.value = await getHistoryAccounts()
    
    if (successCount === historyAccounts.length) {
      message.success('所有账户刷新成功')
    } else {
      message.warning(`成功刷新 ${successCount}/${historyAccounts.length} 个账户`)
    }
  } catch (error) {
    message.error('刷新使用情况失败')
    console.error('刷新使用情况失败:', error)
  } finally {
    loading.value = false
  }
}

async function handleSwitch(account: HistoryAccount) {
  try {
    switchLoadingMap.value[account.email] = true
    
    // 获取当前账户信息并保存到历史记录
    const currentAccount = await getMachineIds()
    if (currentAccount.currentAccount && currentAccount.cursorToken) {
      // 检查当前账户是否已在历史记录中
      const existingAccount = accounts.value.find(a => a.email === currentAccount.currentAccount)
      if (!existingAccount) {
        // 获取当前账户的使用情况
        try {
          const usage = await getUsage(currentAccount.cursorToken)
          const newHistoryAccount: HistoryAccount = {
            email: currentAccount.currentAccount,
            token: currentAccount.cursorToken,
            machineCode: currentAccount.machineId,
            gpt4Count: usage['gpt-4']?.numRequests || 0,
            gpt35Count: usage['gpt-3.5-turbo']?.numRequests || 0,
            gpt4MaxUsage: usage['gpt-4']?.maxRequestUsage || null,
            gpt35MaxUsage: usage['gpt-3.5-turbo']?.maxRequestUsage || null,
            lastUsed: Date.now()
          }
          // 保存到历史记录
          await saveAccountToHistory(newHistoryAccount)
          accounts.value = await getHistoryAccounts()
        } catch (error) {
          console.error('获取当前账户使用情况失败:', error)
        }
      }
    }

    // 继续原有的切换逻辑
    const isRunning = await checkCursorRunning()
    if (isRunning) {
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
    console.error('切换账户失败:', error)
  } finally {
    switchLoadingMap.value[account.email] = false
  }
}

async function handleRemove(account: HistoryAccount) {
  try {
    deleteLoadingMap.value[account.email] = true
    await removeHistoryAccount(account.email)
    accounts.value = accounts.value.filter(a => a.email !== account.email)
    message.success('删除成功')
  } catch (error) {
    message.error('删除失败')
    console.error('删除账户失败:', error)
  } finally {
    deleteLoadingMap.value[account.email] = false
  }
}

// 清理高使用量账户
async function handleClearHighUsageAccounts() {
  if (highUsageAccounts.value.length === 0) {
    message.info('没有高使用量账户需要清理');
    return;
  }
  
  try {
    clearLoading.value = true;
    
    // 并发删除高使用量账户
    const deletePromises = highUsageAccounts.value.map(account => 
      removeHistoryAccount(account.email)
    );
    
    await Promise.all(deletePromises);
    
    // 更新账户列表
    accounts.value = accounts.value.filter(account => {
      const gpt4MaxUsage = account.gpt4MaxUsage || 500;
      const gpt4Usage = account.gpt4Count / gpt4MaxUsage * 100;
      return gpt4Usage < 90;
    });
    
    message.success(`成功清理 ${highUsageAccounts.value.length} 个高使用量账户`);
  } catch (error) {
    message.error('清理高使用量账户失败');
    console.error('清理高使用量账户失败:', error);
  } finally {
    clearLoading.value = false;
  }
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
    console.error('切换账户失败:', error)
  } finally {
    if (pendingAccount.value) {
      switchLoadingMap.value[pendingAccount.value.email] = false
    }
    pendingAccount.value = null
  }
}

onMounted(async () => {
  loading.value = true
  try {
    // 同步本地历史账户到后端
    await syncLocalAccountsToBackend()
    
    // 从后端获取历史账户
    accounts.value = await getHistoryAccounts()
    
    // 自动刷新所有账户数据
    await refreshUsage()
  } catch (error) {
    console.error('加载历史账户失败:', error)
    message.error('加载历史账户失败')
  } finally {
    loading.value = false
  }
  
  window.addEventListener('force_kill_cursor', async (e: Event) => {
    const detail = (e as CustomEvent).detail as PendingForceKillAction
    if (detail.type === 'account') {
      await handleForceKill()
    }
  })
})
</script>

<template>
  <n-space vertical :size="24">
    <n-card title="历史账户">
      <template #header-extra>
        <n-space>
          <n-button 
            @click="refreshUsage" 
            :loading="loading"
            type="primary"
          >
            刷新所有账户
          </n-button>
          <n-button 
            @click="handleClearHighUsageAccounts" 
            :loading="clearLoading"
            type="error"
            :disabled="highUsageAccounts.length === 0"
          >
            清理高使用量账户 ({{ highUsageAccounts.length }})
          </n-button>
        </n-space>
      </template>
      <n-data-table
        :columns="columns"
        :data="accounts"
        :loading="loading"
        :bordered="false"
        :pagination="{
          pageSize: 10
        }"
      />
    </n-card>
  </n-space>
  
  <n-modal
    v-model:show="showCursorRunningModal"
    preset="dialog"
    title="Cursor 正在运行"
    content="检测到 Cursor 正在运行，需要先关闭 Cursor 才能切换账户。是否强制关闭 Cursor 并切换账户？"
    positive-text="强制关闭并切换"
    negative-text="取消"
    @positive-click="handleForceKill"
  />
</template> 