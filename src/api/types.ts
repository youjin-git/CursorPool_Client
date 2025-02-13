// 基础响应类型
interface BaseResponse {
  status: 'success' | 'error'
  message: string
}

// 1. 用户类 API
export interface LoginResponse extends BaseResponse {
  api_key: string
}

export interface ActivateResponse extends BaseResponse {
  data?: {
    expire_time: number
    level: number
    daily_count: number
  }
}

export interface ChangePasswordResponse extends BaseResponse {
  data?: {
    api_key: string
  }
}

export interface UserInfoResponse extends BaseResponse {
  data?: {
    daily_count: number
    daily_used_count: number
    expire_time: number
    level: number
  }
}

// 2. 账户类 API
export interface AccountInfoResponse extends BaseResponse {
  data?: {
    email: string
    token: string
    daily_used: number
    daily_limit: number
  }
}

// 错误类型
export interface ApiError {
  status: 'error'
  message: string
  code?: string // 错误码
}

// 通用分页参数
export interface PaginationParams {
  page: number
  pageSize: number
}

// 通用分页响应
export interface PaginationResponse<T> {
  total: number
  items: T[]
}

// 操作记录类型
export interface OperationRecord {
  id: number
  type: string
  detail: string
  timestamp: string
  operator: string
}

// 操作记录响应
export interface OperationRecordsResponse extends BaseResponse {
  data?: PaginationResponse<OperationRecord>
} 