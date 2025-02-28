import { ref, computed } from 'vue'
import type { VersionInfo } from '@/api/types'
import type { PendingForceKillAction } from '../types'
import { useDeviceInfo } from './useDeviceInfo'

// 创建单例状态
const loading = ref(true)
const machineCodeLoading = ref(false)
const accountSwitchLoading = ref(false)
const quickChangeLoading = ref(false)
const showUpdateModal = ref(false)
const showCursorRunningModal = ref(false)
const showAdminPrivilegeModal = ref(false)
const showInsufficientCreditsModal = ref(false)
const pendingForceKillAction = ref<PendingForceKillAction | null>(null)
const pendingCreditAction = ref<'account' | 'quick' | null>(null)
const versionInfo = ref<VersionInfo | null>(null)
const applyHookLoading = ref(false)

export function useDashboardState() {
  const { deviceInfo } = useDeviceInfo()
  
  // 计算用户当前积分
  const userCredits = computed(() => {
    if (!deviceInfo.value?.userInfo) {
      return 0
    }
    return (deviceInfo.value.userInfo.totalCount - deviceInfo.value.userInfo.usedCount) * 50
  })

  return {
    loading,
    machineCodeLoading,
    accountSwitchLoading,
    quickChangeLoading,
    showUpdateModal,
    showCursorRunningModal,
    showAdminPrivilegeModal,
    showInsufficientCreditsModal,
    pendingForceKillAction,
    pendingCreditAction,
    versionInfo,
    applyHookLoading,
    userCredits
  }
} 