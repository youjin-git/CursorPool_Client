import { ref } from 'vue'
import { getUserInfo, getMachineIds, getUsage, checkHookStatus } from '@/api'
import type { DeviceInfoState } from '../types'

// 创建单例状态
const deviceInfo = ref<DeviceInfoState>({
  machineCode: '',
  currentAccount: '',
  cursorToken: '',
  userInfo: null,
  cursorInfo: {
    userInfo: null,
    usage: null
  },
  hookStatus: null
})

export function useDeviceInfo() {
  // 获取用户信息
  const fetchUserInfo = async () => {
    try {
      const apiKey = localStorage.getItem('apiKey')
      if (!apiKey) {
        throw new Error('未找到 API Key')
      }
      const info = await getUserInfo(apiKey)
      deviceInfo.value.userInfo = info
    } catch (error) {
      localStorage.removeItem('apiKey')
      console.error('获取用户信息失败:', error)
    }
  }

  // 获取机器码
  const fetchMachineIds = async () => {
    try {
      const result = await getMachineIds()

      deviceInfo.value.machineCode = result.machineId
      deviceInfo.value.currentAccount = result.currentAccount
      deviceInfo.value.cursorToken = result.cursorToken
      
      // 获取 Hook 状态
      deviceInfo.value.hookStatus = await checkHookStatus()
    } catch (error) {
      console.error('获取机器码失败:', error)
    }
  }

  // 获取 Cursor 账户信息
  const fetchCursorInfo = async () => {
    try {
      const token = deviceInfo.value.cursorToken
      if (!token) {
        return
      }

      const usageData = await getUsage(token)
      
      deviceInfo.value.cursorInfo = {
        userInfo: {
          email: deviceInfo.value.currentAccount,
          email_verified: true,
          name: deviceInfo.value.currentAccount.split('@')[0],
          sub: '',
          updatedAt: new Date().toISOString(),
          picture: null
        },
        usage: usageData
      }
    } catch (error) {
      console.error('获取 Cursor 账户信息失败:', error)
    }
  }

  return {
    deviceInfo,
    fetchUserInfo,
    fetchMachineIds,
    fetchCursorInfo
  }
} 