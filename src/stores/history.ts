import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useCursorStore } from './cursor'
import type { HistoryRecord, HistoryAccount } from './types'
import type { HistoryRecords } from '../types/history'
import { getHistoryList, syncLocalHistoryToBackend } from '../utils/history'

export const useHistoryStore = defineStore('history', () => {
  // 状态
  const records = ref<HistoryRecords>([])
  const isLoading = ref(false)
  const error = ref('')
  const operationHistory = ref<HistoryRecord[]>([])
  const accountHistory = ref<HistoryAccount[]>([])

  // Getters
  const sortedRecords = computed(() => {
    return [...records.value].sort((a, b) => b.id - a.id)
  })

  // 按类型过滤记录
  const filterByType = (type: string) => {
    return sortedRecords.value.filter(record => record.type === type)
  }

  // 过滤账户历史记录
  const filteredAccounts = computed(() => {
    const cursorStore = useCursorStore()
    const currentEmail = cursorStore.currentAccount
    
    return accountHistory.value.filter(account => account.email !== currentEmail)
  })

  // Actions
  /**
   * 添加历史记录
   */
  function addHistoryRecord(type: string, detail: string, operator = 'System') {
    const newRecord: HistoryRecord = {
      id: Date.now().toString(),
      type,
      detail,
      time: Date.now(),
      operator
    }
    
    operationHistory.value.unshift(newRecord)
    saveHistoryToStorage()
  }

  /**
   * 清除历史记录
   */
  function clearHistory() {
    operationHistory.value = []
    saveHistoryToStorage()
  }

  /**
   * 保存账户到历史
   */
  function saveAccountToHistory(account: HistoryAccount) {
    // 检查是否已存在相同邮箱的账户
    const existingIndex = accountHistory.value.findIndex(a => a.email === account.email)
    
    if (existingIndex !== -1) {
      // 更新现有账户
      accountHistory.value[existingIndex] = {
        ...account,
        lastUsed: Date.now()
      }
    } else {
      // 添加新账户
      accountHistory.value.push({
        ...account,
        lastUsed: Date.now()
      })
    }
    
    // 限制历史账户数量为20个
    if (accountHistory.value.length > 20) {
      accountHistory.value = accountHistory.value
        .sort((a, b) => b.lastUsed - a.lastUsed)
        .slice(0, 20)
    }
    
    saveAccountsToStorage()
  }

  /**
   * 删除历史账户
   */
  function deleteAccount(email: string) {
    accountHistory.value = accountHistory.value.filter(a => a.email !== email)
    saveAccountsToStorage()
  }

  /**
   * 切换到历史账户
   */
  async function switchToHistoryAccount(account: HistoryAccount, forceKill: boolean = false) {
    const cursorStore = useCursorStore()
    
    try {
      isLoading.value = true
      
      // 更新账户的最后使用时间
      saveAccountToHistory({
        ...account,
        lastUsed: Date.now()
      })
      
      // 调用 CursorStore 的切换账户方法
      await cursorStore.switchCursorAccount(account.email, account.token, forceKill)
      
      // 添加历史记录
      addHistoryRecord('账户切换', `切换到账户: ${account.email}`)
      
      return true
    } catch (error) {
      console.error('切换到历史账户失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 保存历史记录到本地存储
   */
  function saveHistoryToStorage() {
    localStorage.setItem('operation_history', JSON.stringify(operationHistory.value))
  }

  /**
   * 保存账户历史到本地存储
   */
  function saveAccountsToStorage() {
    try {
      localStorage.setItem('history_accounts', JSON.stringify(accountHistory.value))
    } catch (error) {
      console.error('保存账户历史失败:', error)
    }
  }

  /**
   * 从本地存储加载历史记录
   */
  function loadHistoryFromStorage() {
    try {
      const savedHistory = localStorage.getItem('operation_history')
      if (savedHistory) {
        operationHistory.value = JSON.parse(savedHistory)
      }
      
      const savedAccounts = localStorage.getItem('history_accounts')
      if (savedAccounts) {
        accountHistory.value = JSON.parse(savedAccounts)
      }
    } catch (error) {
      console.error('加载历史记录失败:', error)
    }
  }

  /**
   * 初始化历史记录
   */
  function init() {
    loadHistoryFromStorage()
  }

  // 加载历史记录
  async function loadHistoryRecords() {
    isLoading.value = true
    error.value = ''
    
    try {
      // 先尝试同步本地历史记录到后端
      await syncLocalHistoryToBackend()
      
      // 然后从后端获取所有历史记录
      records.value = await getHistoryList()
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载历史记录失败'
      console.error('加载历史记录失败:', err)
    } finally {
      isLoading.value = false
    }
  }
  
  // 监听历史记录更新事件
  function setupHistoryListener() {
    window.addEventListener('history_updated', async () => {
      await loadHistoryRecords()
    })
    
    return () => {
      window.removeEventListener('history_updated', async () => {
        await loadHistoryRecords()
      })
    }
  }

  return {
    records,
    isLoading,
    error,
    sortedRecords,
    filterByType,
    loadHistoryRecords,
    setupHistoryListener,
    operationHistory,
    accountHistory,
    filteredAccounts,
    addHistoryRecord,
    clearHistory,
    saveAccountToHistory,
    deleteAccount,
    switchToHistoryAccount,
    init
  }
}) 