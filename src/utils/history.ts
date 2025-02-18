import type { OperationRecord, HistoryRecords } from '../types/history'

const HISTORY_KEY = 'operation_history'

export function addHistoryRecord(type: string, detail: string) {
  const history: HistoryRecords = JSON.parse(localStorage.getItem(HISTORY_KEY) || '[]')
  
  const newRecord: OperationRecord = {
    id: Date.now(),
    type,
    detail,
    timestamp: new Date().toLocaleString(),
    operator: 'System'
  }
  
  history.unshift(newRecord) // 新记录添加到开头
  localStorage.setItem(HISTORY_KEY, JSON.stringify(history))
  
  // 触发更新事件
  window.dispatchEvent(new Event('history_updated'))
} 