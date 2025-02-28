import { useMessage } from 'naive-ui'
import { useI18n } from '../../../locales'
import { applyHook, checkCursorRunning, checkHookStatus } from '@/api'
import { useDashboardState } from './useDashboardState'
import { useDeviceInfo } from './useDeviceInfo'
import { addHistoryRecord } from '@/utils/history'

export function useHookActions() {
  const message = useMessage()
  const { i18n } = useI18n()
  const { showCursorRunningModal, pendingForceKillAction, applyHookLoading } = useDashboardState()
  const { deviceInfo } = useDeviceInfo()

  // 处理注入
  const handleApplyHook = async () => {
    try {
      applyHookLoading.value = true
      
      // 检查 Cursor 是否在运行
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { 
          type: 'hook'
        }
        return
      }
      
      await applyHook(false)
      message.success(i18n.value.systemControl.messages.applyHookSuccess)
      
      // 重新检查 Hook 状态，确保状态更新
      deviceInfo.value.hookStatus = await checkHookStatus()
      
      // 触发全局刷新事件
      window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
      
      addHistoryRecord('系统控制', i18n.value.systemControl.history.applyHook)
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error)
      if (errorMsg === 'Cursor进程正在运行, 请先关闭Cursor') {
        showCursorRunningModal.value = true
        pendingForceKillAction.value = { 
          type: 'hook',
        }
        return
      }
      message.error(error instanceof Error ? error.message : '注入失败')
    } finally {
      applyHookLoading.value = false
    }
  }

  return {
    applyHookLoading,
    handleApplyHook
  }
} 