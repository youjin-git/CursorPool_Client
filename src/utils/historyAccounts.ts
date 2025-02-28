import type { HistoryAccount } from '@/types/history'

const STORAGE_KEY = 'history_accounts'

export function saveAccountToHistory(account: HistoryAccount) {
  const history = getHistoryAccounts()
  const index = history.findIndex(a => a.email === account.email)
  
  if (index >= 0) {
    history[index] = account
  } else {
    history.push(account)
  }
  
  localStorage.setItem(STORAGE_KEY, JSON.stringify(history))
}

export function getHistoryAccounts(): HistoryAccount[] {
  const data = localStorage.getItem(STORAGE_KEY)
  return data ? JSON.parse(data) : []
}

export function removeHistoryAccount(email: string) {
  const history = getHistoryAccounts()
  const filtered = history.filter(a => a.email !== email)
  localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered))
} 