import type { HistoryAccount } from '@/types/history'
import { saveHistoryAccount as apiSaveHistoryAccount, getHistoryAccounts as apiGetHistoryAccounts, removeHistoryAccount as apiRemoveHistoryAccount } from '@/api'
import type { HistoryAccountRecord } from '@/api/types'

const STORAGE_KEY = 'history_accounts'

/**
 * 将前端HistoryAccount转换为后端HistoryAccountRecord
 */
function convertToBackendAccount(account: HistoryAccount): HistoryAccountRecord {
  return {
    email: account.email,
    token: account.token,
    machine_code: account.machineCode,
    gpt4_count: account.gpt4Count,
    gpt35_count: account.gpt35Count,
    last_used: account.lastUsed,
    gpt4_max_usage: account.gpt4MaxUsage,
    gpt35_max_usage: account.gpt35MaxUsage
  }
}

/**
 * 将后端HistoryAccountRecord转换为前端HistoryAccount
 */
function convertToFrontendAccount(account: HistoryAccountRecord): HistoryAccount {
  return {
    email: account.email,
    token: account.token,
    machineCode: account.machine_code,
    gpt4Count: account.gpt4_count,
    gpt35Count: account.gpt35_count,
    lastUsed: account.last_used,
    gpt4MaxUsage: account.gpt4_max_usage,
    gpt35MaxUsage: account.gpt35_max_usage
  }
}

/**
 * 保存账户到历史记录
 */
export async function saveAccountToHistory(account: HistoryAccount) {
  try {
    // 保存到后端
    await apiSaveHistoryAccount(convertToBackendAccount(account))
  } catch (error) {
    console.error('保存账户到后端失败，回退到本地存储:', error)
    
    // 如果后端保存失败，回退到本地存储
    const history = getHistoryAccountsFromLocal()
    const index = history.findIndex(a => a.email === account.email)
    
    if (index >= 0) {
      history[index] = account
    } else {
      history.push(account)
    }
    
    localStorage.setItem(STORAGE_KEY, JSON.stringify(history))
  }
}

/**
 * 获取历史账户列表
 */
export async function getHistoryAccounts(): Promise<HistoryAccount[]> {
  try {
    // 从后端获取
    const accounts = await apiGetHistoryAccounts()
    return accounts.map(convertToFrontendAccount)
  } catch (error) {
    console.error('从后端获取历史账户失败，回退到本地存储:', error)
    
    // 如果后端获取失败，回退到本地存储
    return getHistoryAccountsFromLocal()
  }
}

/**
 * 从本地存储获取历史账户
 */
function getHistoryAccountsFromLocal(): HistoryAccount[] {
  const data = localStorage.getItem(STORAGE_KEY)
  return data ? JSON.parse(data) : []
}

/**
 * 删除历史账户
 */
export async function removeHistoryAccount(email: string) {
  try {
    // 从后端删除
    await apiRemoveHistoryAccount(email)
  } catch (error) {
    console.error('从后端删除历史账户失败，回退到本地存储:', error)
    
    // 如果后端删除失败，回退到本地存储
    const history = getHistoryAccountsFromLocal()
    const filtered = history.filter(a => a.email !== email)
    localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered))
  }
}

/**
 * 同步本地历史账户到后端
 */
export async function syncLocalAccountsToBackend() {
  const localAccounts = localStorage.getItem(STORAGE_KEY)
  
  if (!localAccounts) {
    return // 没有本地历史账户，不需要同步
  }
  
  try {
    const accounts: HistoryAccount[] = JSON.parse(localAccounts)
    
    if (accounts.length === 0) {
      // 空记录，直接清除本地存储
      localStorage.removeItem(STORAGE_KEY)
      return
    }
    
    // 逐个保存到后端
    for (const account of accounts) {
      await apiSaveHistoryAccount(convertToBackendAccount(account))
    }
    
    console.log(`成功同步 ${accounts.length} 个本地历史账户到后端`)
  } catch (error) {
    console.error('同步本地历史账户到后端失败:', error)
    // 即使同步失败，也清除本地存储，避免重复同步
  } finally {
    // 无论成功失败，都清除本地存储
    localStorage.removeItem(STORAGE_KEY)
  }
} 