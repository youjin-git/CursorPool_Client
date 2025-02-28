export interface OperationRecord {
  id: number
  type: string
  detail: string
  timestamp: string
  operator: string
}

export type HistoryRecords = OperationRecord[]

export interface HistoryAccount {
  email: string
  token: string
  machineCode: string  // 机器码字段
  gpt4Count: number  // 高级模型使用次数
  gpt35Count: number // 普通模型使用次数
  lastUsed: number   // 最后使用时间戳
} 