import type { UserInfo, CursorUserInfo, CursorUsageInfo } from '@/api/types'

export interface DeviceInfoState {
  machineCode: string
  currentAccount: string
  cursorToken: string
  userInfo: UserInfo | null
  cursorInfo: {
    userInfo: CursorUserInfo | null
    usage: CursorUsageInfo | null
  }
  hookStatus: boolean | null
}

export interface PendingForceKillAction {
  type: 'machine' | 'account' | 'quick' | 'hook'
  params?: any
} 