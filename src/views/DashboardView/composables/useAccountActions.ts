import { useMessage } from 'naive-ui'
import { useI18n } from '../../../locales'
import { 
  resetMachineId, 
  switchAccount, 
  getAccount, 
  checkCursorRunning,
  checkHookStatus
} from '@/api'
import { addHistoryRecord } from '../../../utils/history'
import { useDashboardState } from './useDashboardState'
import { useDeviceInfo } from './useDeviceInfo'
import { computed, ref } from 'vue'

export function useAccountActions() {
  const message = useMessage()
  const { i18n } = useI18n()
  
  const { 
    showCursorRunningModal, 
    showCCStatusModal, 
    pendingForceKillAction,
    originalActionBeforeHook
  } = useDashboardState()
  
  const { deviceInfo, fetchUserInfo, fetchMachineIds, fetchCursorInfo } = useDeviceInfo()
  
  // 添加新的状态
  const showInsufficientCreditsModal = ref(false)
  const pendingCreditAction = ref<'account' | 'quick' | null>(null)
  
  // 计算用户当前积分
  const userCredits = computed(() => {
    if (!deviceInfo.value.userInfo) return 0
    return (deviceInfo.value.userInfo.totalCount - deviceInfo.value.userInfo.usedCount) * 50
  })

  // 修改机器码
  const handleMachineCodeChange = async () => {
    // 先检查 CC 状态
    try {
      // 重新检查 Hook 状态，确保获取最新状态
      const hookStatus = await checkHookStatus()
      deviceInfo.value.hookStatus = hookStatus
      
      if (!deviceInfo.value.hookStatus) {
        originalActionBeforeHook.value = { type: 'machine' }
        showCCStatusModal.value = true
        return
      }

      await resetMachineId(false)
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
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { type: 'machine' }
        return
      }
      message.error(i18n.value.dashboard.machineChangeFailed)
    }
  }

  // 账户切换
  const handleAccountSwitch = async () => {
    try {
      // 重新检查 Hook 状态，确保获取最新状态
      const hookStatus = await checkHookStatus()
      deviceInfo.value.hookStatus = hookStatus
      
      // 先检查 CC 状态
      if (!deviceInfo.value.hookStatus) {
        originalActionBeforeHook.value = { type: 'account' }
        showCCStatusModal.value = true
        return
      }

      // 检查 Cursor 是否在运行
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { type: 'account' }
        return
      }
      
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
  const executeAccountSwitch = async () => {
    try {
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
      
      await switchAccount(accountInfo.email, accountInfo.token, false)
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
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { type: 'account' }
        return
      }
      console.error('切换账户失败:', error)
      message.error(i18n.value.dashboard.accountChangeFailed)
    }
  }

  // 一键切换
  const handleQuickChange = async () => {
    try {
      // 重新检查 Hook 状态，确保获取最新状态
      const hookStatus = await checkHookStatus()
      deviceInfo.value.hookStatus = hookStatus
      
      // 先检查 CC 状态
      if (!deviceInfo.value.hookStatus) {
        originalActionBeforeHook.value = { type: 'quick' }
        showCCStatusModal.value = true
        return
      }

      // 检查 Cursor 是否在运行
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { type: 'quick' }
        return
      }
      
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
  const executeQuickChange = async () => {
    try {
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
      
      await switchAccount(accountInfo.email, accountInfo.token, false)
      message.success(i18n.value.dashboard.accountChangeSuccess)
      addHistoryRecord(
        '账户切换',
        `切换到账户: ${accountInfo.email} 扣除50积分`
      )
      
      // 先获取机器码信息，这样可以更新 cursorToken
      await fetchMachineIds()
      
      // 然后再修改机器码
      await resetMachineId(false)
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
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { type: 'quick' }
        return
      }
      message.error(i18n.value.common.copyFailed)
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

  // 监听事件，在注入成功后继续执行原始操作
  window.addEventListener('continue_original_action', ((event: CustomEvent) => {
    const { actionType } = event.detail
    if (actionType === 'machine') {
      handleMachineCodeChange()
    } else if (actionType === 'account') {
      handleAccountSwitch()
    } else if (actionType === 'quick') {
      handleQuickChange()
    }
  }) as EventListener)

  return {
    handleMachineCodeChange,
    handleAccountSwitch,
    handleQuickChange,
    executeAccountSwitch,
    executeQuickChange,
    showInsufficientCreditsModal,
    pendingCreditAction,
    userCredits,
    handleActivateSuccess
  }
} 