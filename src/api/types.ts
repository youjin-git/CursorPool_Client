// 通用响应类型
export interface ApiResponse<T> {
  status: string
  message: string
  data?: T
}

// 用户信息
export interface UserInfo {
  daily_count: number
  daily_used_count: number
  expire_time: number
  level: number
  is_expired: boolean
  username: string
  email?: string
  activated: boolean
}

// 账户信息
export interface AccountInfo {
  email: string
  token: string
  dailyUsed: number
  dailyLimit: number
}

// 账户详细信息
export interface AccountDetail {
  email: string
  user_id: string
  token: string
  daily_used: number
  daily_limit: number
}

// 登录请求
export interface LoginRequest {
  username: string
  password: string
  deviceId: string
  smsCode?: string
}

// 登录响应
export interface LoginResponse {
  api_key?: string
}

// 检查用户请求
export interface CheckUserRequest {
  username: string
}

// 检查用户响应
export interface CheckUserResponse {
  exists: boolean
  need_code: boolean
}

// 发送验证码请求
export interface SendCodeRequest {
  username: string
}

// 发送验证码响应
export interface SendCodeResponse {
  expire_in: number
}

// 激活请求
export interface ActivateRequest {
  code: string
}

// 激活响应
export interface ActivateResponse {
  expire_time: number
  level: number
}

// 修改密码请求
export interface ChangePasswordRequest {
  oldPassword: string
  newPassword: string
}

// 版本信息响应
export interface VersionInfo {
  version: string
  forceUpdate: boolean
  downloadUrl: string
  changeLog: string
}

// 公告信息响应
export interface PublicInfo {
  type: string
  closeable: boolean
  props: PublicInfoProps
  actions: PublicInfoAction[]
}

export interface PublicInfoProps {
  title: string
  description: string
}

export interface PublicInfoAction {
  type: string
  text: string
  url: string
}

// GPT 模型使用情况
export interface GptModelUsage {
  numRequests: number
  numRequestsTotal: number
  numTokens: number
  maxRequestUsage?: number
  maxTokenUsage?: number
}

// 使用情况响应
export interface UsageInfo extends CursorUsageInfo {}

// 用户信息响应（Cursor平台）
export interface UserInfoResponse extends CursorUserInfo {}

// Cursor 用户信息
export interface CursorUserInfo {
  email: string
  email_verified: boolean
  name: string
  sub: string
  updated_at: string
  picture: string | null
}

// Cursor 模型使用情况
export interface CursorModelUsage {
  numRequests: number
  numRequestsTotal: number
  numTokens: number
  maxRequestUsage: number | null
  maxTokenUsage: number | null
}

// Cursor 使用情况
export interface CursorUsageInfo {
  'gpt-4': CursorModelUsage
  'gpt-3.5-turbo': CursorModelUsage
  'gpt-4-32k': CursorModelUsage
  startOfMonth: string
}

// 设备信息
export interface DeviceInfo {
  machineCode: string
  currentAccount: string
  userInfo: UserInfo | null
  cursorInfo: {
    userInfo: CursorUserInfo | null
    usage: CursorUsageInfo | null
  } | null
}

// 机器码信息
export interface MachineInfo {
  machine_id: string
  current_account: string
}