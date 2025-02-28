import { useMessage } from 'naive-ui'
import { useI18n } from '../../../locales'
import { 
  resetMachineId, 
  switchAccount, 
  getAccount, 
  checkCursorRunning,
  applyHook,
  checkHookStatus
} from '@/api'
import { addHistoryRecord } from '../../../utils/history'
import { useDashboardState } from './useDashboardState'
import { useDeviceInfo } from './useDeviceInfo'
import { saveAccountToHistory } from '@/utils/historyAccounts'
import type { HistoryAccount } from '@/types/history'
import type { PendingForceKillAction } from '../types'
import { inject, onMounted, onUnmounted } from 'vue'

export function useAccountActions() {
  const message = useMessage()
  const { i18n } = useI18n()
  
  const { 
    userCredits,
    showInsufficientCreditsModal,
    pendingCreditAction,
    machineCodeLoading,
    accountSwitchLoading,
    quickChangeLoading
  } = useDashboardState()
  
  const { deviceInfo, fetchUserInfo, fetchMachineIds, fetchCursorInfo } = useDeviceInfo()
  
  const showCursorModal = inject<(action: PendingForceKillAction) => void>('showCursorModal')
  
  // 监听强制关闭事件
  onMounted(() => {
    window.addEventListener('force_kill_cursor', async (e: Event) => {
      const detail = (e as CustomEvent).detail as PendingForceKillAction
      if (detail.type === 'machine') {
        await handleMachineCodeForceKill()
      } else if (detail.type === 'account') {
        await executeAccountSwitch(true)
      } else if (detail.type === 'quick') {
        await executeQuickChange(true)
      }
    })
  })

  // 强制关闭后的机器码修改
  const handleMachineCodeForceKill = async () => {
    try {
      machineCodeLoading.value = true
      await ensureHookApplied()
      await resetMachineId({ forceKill: true })
      message.success(i18n.value.dashboard.machineChangeSuccess)
      addHistoryRecord(
        '机器码修改',
        `修改机器码: ${deviceInfo.value.machineCode}`
      )
      await fetchMachineIds()
      window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
    } catch (error) {
      message.error(i18n.value.dashboard.machineChangeFailed)
    } finally {
      machineCodeLoading.value = false
    }
  }

  // 检查并自动注入
  const ensureHookApplied = async () => {
    const hookStatus = await checkHookStatus()
    if (!hookStatus) {
      await applyHook(false)
      deviceInfo.value.hookStatus = true
    }
  }

  // 修改机器码
  const handleMachineCodeChange = async () => {
    try {
      machineCodeLoading.value = true
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showCursorModal?.({ type: 'machine' })
        return
      }
      await ensureHookApplied()

      await resetMachineId({ forceKill: false })
      message.success(i18n.value.dashboard.machineChangeSuccess)
      addHistoryRecord(
        '机器码修改',
        `修改机器码: ${deviceInfo.value.machineCode}`
      )
      await fetchMachineIds()
      // 触发全局刷新事件
      window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error)
      if (errorMsg === 'Cursor进程正在运行, 请先关闭Cursor') {
        showCursorModal?.({ type: 'machine' })
        return
      }
      message.error(i18n.value.dashboard.machineChangeFailed)
    } finally {
      machineCodeLoading.value = false
    }
  }

  // 账户切换
  const handleAccountSwitch = async () => {
    try {
      // 保存当前账户到历史记录
      if (deviceInfo.value.cursorInfo?.userInfo) {
        const historyAccount: HistoryAccount = {
          email: deviceInfo.value.cursorInfo.userInfo.email,
          token: deviceInfo.value.cursorToken,
          machineCode: deviceInfo.value.machineCode,
          gpt4Count: deviceInfo.value.cursorInfo.usage?.['gpt-4']?.numRequests || 0,
          gpt35Count: deviceInfo.value.cursorInfo.usage?.['gpt-3.5-turbo']?.numRequests || 0,
          lastUsed: Date.now()
        }
        saveAccountToHistory(historyAccount)
      }

      await ensureHookApplied()

      // 检查 Cursor 是否在运行
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showCursorModal?.({ type: 'account' })
        return
      }
      
      // 确保先获取最新的用户信息
      await fetchUserInfo()
      
      // 检查积分是否足够
      if (userCredits.value < 50) {
        showInsufficientCreditsModal.value = true
        pendingCreditAction.value = 'account'
        return
      }

      await executeAccountSwitch()
    } catch (error) {
      message.error('操作失败: ' + (error instanceof Error ? error.message : String(error)))
    }
  }

  // 执行账户切换
  const executeAccountSwitch = async (forceKill: boolean = false) => {
    try {
      accountSwitchLoading.value = true
      const apiKey = localStorage.getItem('apiKey')
      if (!apiKey) {
        message.error(i18n.value.message.pleaseInputEmail)
        return
      }

      // 获取账号信息并执行实际的切换
      const accountInfo = await getAccount(apiKey)
      
      if (!accountInfo.email || !accountInfo.token) {
        message.error(i18n.value.dashboard.accountChangeFailed)
        return
      }
      
      await switchAccount(accountInfo.email, accountInfo.token, forceKill)
      message.success(i18n.value.dashboard.accountChangeSuccess)
      addHistoryRecord(
        '账户切换',
        `切换到账户: ${accountInfo.email} 扣除50积分`
      )
      
      // 先获取机器码信息，这样可以更新 cursorToken
      await fetchMachineIds()
      
      // 然后再获取用户信息和 Cursor 信息
      await fetchUserInfo()
      await fetchCursorInfo()
      
      // 触发全局刷新事件
      window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error)
      if (errorMsg === 'Cursor进程正在运行, 请先关闭Cursor') {
        showCursorModal?.({ type: 'account' })
        return
      }
      console.error('切换账户失败:', error)
      message.error(i18n.value.dashboard.accountChangeFailed)
    } finally {
      accountSwitchLoading.value = false
    }
  }

  // 一键切换
  const handleQuickChange = async () => {
    try {
      // 保存当前账户到历史记录
      if (deviceInfo.value.cursorInfo?.userInfo) {
        const historyAccount: HistoryAccount = {
          email: deviceInfo.value.cursorInfo.userInfo.email,
          token: deviceInfo.value.cursorToken,
          machineCode: deviceInfo.value.machineCode,
          gpt4Count: deviceInfo.value.cursorInfo.usage?.['gpt-4']?.numRequests || 0,
          gpt35Count: deviceInfo.value.cursorInfo.usage?.['gpt-3.5-turbo']?.numRequests || 0,
          lastUsed: Date.now()
        }
        saveAccountToHistory(historyAccount)
      }

      await ensureHookApplied()

      // 检查 Cursor 是否在运行
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showCursorModal?.({ type: 'quick' })
        return
      }
      
      // 确保先获取最新的用户信息
      await fetchUserInfo()
      
      // 检查积分是否足够
      if (userCredits.value < 50) {
        showInsufficientCreditsModal.value = true
        pendingCreditAction.value = 'quick'
        return
      }

      await executeQuickChange()
    } catch (error) {
      message.error('操作失败: ' + (error instanceof Error ? error.message : String(error)))
    }
  }

  // 执行一键切换
  const executeQuickChange = async (forceKill: boolean = false) => {
    try {
      quickChangeLoading.value = true
      // 先执行账户切换
      const apiKey = localStorage.getItem('apiKey')
      if (!apiKey) {
        message.error(i18n.value.message.pleaseInputEmail)
        return
      }

      // 获取账号信息并执行实际的切换
      const accountInfo = await getAccount(apiKey)
      
      if (!accountInfo.email || !accountInfo.token) {
        message.error(i18n.value.dashboard.accountChangeFailed)
        return
      }
      
      await switchAccount(accountInfo.email, accountInfo.token, forceKill)
      message.success(i18n.value.dashboard.accountChangeSuccess)
      addHistoryRecord(
        '账户切换',
        `切换到账户: ${accountInfo.email} 扣除50积分`
      )
      
      // 先获取机器码信息，这样可以更新 cursorToken
      await fetchMachineIds()
      
      // 然后再修改机器码
      await resetMachineId({ forceKill: false })
      message.success(i18n.value.dashboard.machineChangeSuccess)
      addHistoryRecord(
        '机器码修改',
        `修改机器码: ${deviceInfo.value.machineCode}`
      )
      
      // 再次获取机器码信息
      await fetchMachineIds()
      
      // 最后获取用户信息和 Cursor 信息
      await fetchUserInfo()
      await fetchCursorInfo()
      
      // 触发全局刷新事件
      window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error)
      if (errorMsg === 'Cursor进程正在运行, 请先关闭Cursor') {
        showCursorModal?.({ type: 'quick' })
        return
      }
      message.error(i18n.value.dashboard.accountChangeFailed)
    } finally {
      quickChangeLoading.value = false
    }
  }
  
  // 处理激活成功
  const handleActivateSuccess = async () => {
    // 重新获取用户信息
    await fetchUserInfo()
    
    // 如果积分已经足够，继续执行之前的操作
    if (userCredits.value >= 50) {
      if (pendingCreditAction.value === 'account') {
        await executeAccountSwitch()
      } else if (pendingCreditAction.value === 'quick') {
        await executeQuickChange()
      }
      pendingCreditAction.value = null
    } else {
      message.info('积分仍然不足，请继续充值或联系客服')
    }
  }

  // 创建具名的事件处理函数
  const handleOriginalAction = ((event: CustomEvent) => {
    const { actionType } = event.detail
    if (actionType === 'machine') {
      handleMachineCodeChange()
    } else if (actionType === 'account') {
      handleAccountSwitch()
    } else if (actionType === 'quick') {
      handleQuickChange()
    }
  }) as EventListener

  onMounted(() => {
    window.addEventListener('continue_original_action', handleOriginalAction)
  })

  onUnmounted(() => {
    window.removeEventListener('continue_original_action', handleOriginalAction)
  })

  return {
    handleMachineCodeChange,
    handleAccountSwitch,
    handleQuickChange,
    executeAccountSwitch,
    executeQuickChange,
    showInsufficientCreditsModal,
    pendingCreditAction,
    userCredits,
    handleActivateSuccess,
    machineCodeLoading,
    accountSwitchLoading,
    quickChangeLoading
  }
} 