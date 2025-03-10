import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { 
  getMachineIds, 
  getUsage, 
  resetMachineId, 
  switchAccount,
  checkHookStatus,
  applyHook,
  restoreHook,
  closeCursor,
  launchCursor
} from '@/api'
import type { UsageInfo, MachineInfo } from '@/api/types'

export const useCursorStore = defineStore('cursor', () => {
  // 状态
  const machineCode = ref('')
  const currentAccount = ref('')
  const cursorToken = ref('')
  const cursorInfo = ref<{
    userInfo: any | null,
    usage: UsageInfo | null
  }>({
    userInfo: null,
    usage: null
  })
  const isLoading = ref(false)
  const hookStatus = ref<boolean | null>(null)

  // Getters
  const gpt4Usage = computed(() => {
    const usage = cursorInfo.value?.usage?.['gpt-4']
    if (!usage) return { used: 0, total: 0, percentage: 0 }
    return {
      used: usage.numRequests || 0,
      total: usage.maxRequestUsage || 0,
      percentage: getUsagePercentage(usage.numRequests, usage.maxRequestUsage)
    }
  })

  const gpt35Usage = computed(() => {
    const usage = cursorInfo.value?.usage?.['gpt-3.5-turbo']
    if (!usage) return { used: 0, total: 0, percentage: 0 }
    return {
      used: usage.numRequests || 0,
      total: usage.maxRequestUsage || 0,
      percentage: getUsagePercentage(usage.numRequests, usage.maxRequestUsage)
    }
  })

  const isHooked = computed(() => hookStatus.value === true)

  // 辅助函数
  function getUsagePercentage(used: number | null | undefined, total: number | null | undefined) {
    if (!used || !total) return 0
    return Math.min(100, Math.round((used / total) * 100))
  }

  // Actions
  /**
   * 获取机器码信息
   * @returns MachineInfo 机器码信息
   */
  async function fetchMachineIds(): Promise<MachineInfo> {
    try {
      isLoading.value = true
      const result = await getMachineIds()
      machineCode.value = result.machineId
      currentAccount.value = result.currentAccount
      cursorToken.value = result.cursorToken
      return result
    } catch (error) {
      console.error('获取机器码失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 获取 Cursor 使用量
   */
  async function fetchCursorUsage() {
    try {
      if (!cursorToken.value) {
        await fetchMachineIds()
      }
      
      if (!cursorToken.value) {
        console.error('未找到 Cursor Token')
        return
      }

      isLoading.value = true
      const usageData = await getUsage(cursorToken.value)
      
      cursorInfo.value = {
        userInfo: {
          email: currentAccount.value,
          email_verified: true,
          name: currentAccount.value.split('@')[0],
          sub: '',
          updated_at: new Date().toISOString(),
          picture: null
        },
        usage: usageData
      }
    } catch (error) {
      console.error('获取 Cursor 使用量失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 重置机器码
   */
  async function resetMachine(params: { forceKill?: boolean, machineId?: string } = {}) {
    try {
      isLoading.value = true
      await resetMachineId(params)
      // 重置成功后刷新数据
      await fetchMachineIds()
      await fetchCursorUsage()
      return true
    } catch (error) {
      console.error('重置机器码失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 切换账户
   */
  async function switchCursorAccount(email: string, token: string, forceKill: boolean = false) {
    try {
      isLoading.value = true
      await switchAccount(email, token, forceKill)
      // 切换成功后刷新数据
      await fetchMachineIds()
      await fetchCursorUsage()
      return true
    } catch (error) {
      console.error('切换账户失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 一键更换（账户+机器码）
   */
  async function quickChange(email: string, token: string, forceKill: boolean = false) {
    try {
      isLoading.value = true
      
      // 先重置机器码
      await resetMachineId({ forceKill })
      
      // 再切换账户
      await switchAccount(email, token, forceKill)
      
      // 刷新数据
      await fetchMachineIds()
      await fetchCursorUsage()
      
      return true
    } catch (error) {
      console.error('一键更换失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 检查 Hook 状态
   */
  async function checkHook() {
    try {
      hookStatus.value = await checkHookStatus()
      return hookStatus.value
    } catch (error) {
      console.error('检查 Hook 状态失败:', error)
      hookStatus.value = null
      throw error
    }
  }

  /**
   * 应用 Hook
   */
  async function applyHookToClient(forceKill: boolean = false) {
    try {
      isLoading.value = true
      await applyHook(forceKill)
      hookStatus.value = true
      return true
    } catch (error) {
      console.error('应用 Hook 失败:', error)
      throw error
    } finally {
      isLoading.value = false
      await checkHook()
    }
  }

  /**
   * 恢复 Hook
   */
  async function restoreHookFromClient(forceKill: boolean = false) {
    try {
      isLoading.value = true
      await restoreHook(forceKill)
      hookStatus.value = false
      return true
    } catch (error) {
      console.error('恢复 Hook 失败:', error)
      throw error
    } finally {
      isLoading.value = false
      await checkHook()
    }
  }

  /**
   * 关闭 Cursor
   */
  async function closeCursorApp() {
    try {
      return await closeCursor()
    } catch (error) {
      console.error('关闭 Cursor 失败:', error)
      throw error
    }
  }

  /**
   * 启动 Cursor
   */
  async function launchCursorApp() {
    try {
      return await launchCursor()
    } catch (error) {
      console.error('启动 Cursor 失败:', error)
      throw error
    }
  }

  /**
   * 初始化 Cursor 数据
   */
  async function initCursorData() {
    try {
      isLoading.value = true
      await fetchMachineIds()
      await fetchCursorUsage()
      await checkHook()
    } catch (error) {
      console.error('初始化 Cursor 数据失败:', error)
    } finally {
      isLoading.value = false
    }
  }

  return {
    // 状态
    machineCode,
    currentAccount,
    cursorToken,
    cursorInfo,
    isLoading,
    hookStatus,
    
    // Getters
    gpt4Usage,
    gpt35Usage,
    isHooked,
    
    // Actions
    fetchMachineIds,
    fetchCursorUsage,
    resetMachine,
    switchCursorAccount,
    quickChange,
    checkHook,
    applyHookToClient,
    restoreHookFromClient,
    closeCursorApp,
    launchCursorApp,
    initCursorData
  }
}) 