import { ref, watch } from 'vue'
import type { VersionInfo } from '@/api/types'
import type { OriginalAction, PendingForceKillAction } from '../types'

// 创建单例状态
const loading = ref(true)
const showUpdateModal = ref(false)
const showCursorRunningModal = ref(false)
const showAdminPrivilegeModal = ref(false)
const showCCStatusModal = ref(false)
const pendingForceKillAction = ref<PendingForceKillAction | null>(null)
const versionInfo = ref<VersionInfo | null>(null)
const applyHookLoading = ref(false)
const originalActionBeforeHook = ref<OriginalAction>({ type: null })

// 添加调试代码
watch(showCCStatusModal, (newVal) => {
  console.log('useDashboardState showCCStatusModal changed', newVal)
})

export function useDashboardState() {
  return {
    loading,
    showUpdateModal,
    showCursorRunningModal,
    showAdminPrivilegeModal,
    showCCStatusModal,
    pendingForceKillAction,
    versionInfo,
    applyHookLoading,
    originalActionBeforeHook
  }
} 