<script setup lang="ts">
import { onMounted, ref, watch, computed } from 'vue'
import { useMessage } from 'naive-ui'
import { NGrid, NGridItem, NSpace } from 'naive-ui'
import { 
  killCursorProcess, 
  waitForCursorClose, 
  checkHookStatus, 
  checkCursorRunning 
} from '@/api'

import UserInfoCard from './components/UserInfoCard.vue'
import UsageStatsCard from './components/UsageStatsCard.vue'
import QuickActionsCard from './components/QuickActionsCard.vue'
import UpdateModal from './components/UpdateModal.vue'
import CursorRunningModal from './components/CursorRunningModal.vue'
import AdminPrivilegeModal from './components/AdminPrivilegeModal.vue'
import CCStatusModal from './components/CCStatusModal.vue'
import DashboardTour from './components/DashboardTour.vue'
import InsufficientCreditsModal from './components/InsufficientCreditsModal.vue'

import { useDashboardState } from './composables/useDashboardState'
import { useDeviceInfo } from './composables/useDeviceInfo'
import { useVersionCheck } from './composables/useVersionCheck'
import { usePrivilegeCheck } from './composables/usePrivilegeCheck'
import { useAccountActions } from './composables/useAccountActions'
import { useHookActions } from './composables/useHookActions'

const message = useMessage()

// 导入状态和方法
const { 
  loading, 
  showUpdateModal, 
  showCursorRunningModal,
  showAdminPrivilegeModal,
  showCCStatusModal,
  pendingForceKillAction,
  versionInfo,
  originalActionBeforeHook
} = useDashboardState()

// 导入设备信息和相关方法
const { 
  deviceInfo, 
  fetchUserInfo, 
  fetchMachineIds, 
  fetchCursorInfo 
} = useDeviceInfo()

// 导入版本检查相关方法
const { 
  checkUpdate, 
  handleDownload, 
  handleLater 
} = useVersionCheck(showUpdateModal, versionInfo)

// 导入权限检查相关方法
const { 
  checkPrivileges, 
  handleExit 
} = usePrivilegeCheck(showAdminPrivilegeModal)

// 导入账户操作相关方法
const {
  handleMachineCodeChange,
  executeAccountSwitch,
  executeQuickChange
} = useAccountActions()

// 导入注入相关方法
const { handleApplyHook } = useHookActions()

// 添加新的状态
const showInsufficientCreditsModal = ref(false)
const pendingCreditAction = ref<'account' | 'quick' | null>(null)

// 计算用户当前积分
const userCredits = computed(() => {
  if (!deviceInfo.value.userInfo) return 0
  return (deviceInfo.value.userInfo.totalCount - deviceInfo.value.userInfo.usedCount) * 50
})

// 修改账户切换方法
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

// 修改一键切换方法
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

// 处理强制关闭
const handleForceKill = async () => {
  try {
    await killCursorProcess()
    message.success('已强制关闭 Cursor')
    
    // 等待进程完全关闭
    await waitForCursorClose()
    
    // 执行原始操作
    if (pendingForceKillAction.value) {
      const { type } = pendingForceKillAction.value
      
      if (type === 'machine') {
        handleMachineCodeChange()
      } else if (type === 'account') {
        handleAccountSwitch()
      } else if (type === 'quick') {
        handleQuickChange()
      } else if (type === 'hook') {
        handleApplyHook(originalActionBeforeHook.value)
      }
      
      // 重置待处理操作
      pendingForceKillAction.value = null
    }
    
    // 关闭模态框
    showCursorRunningModal.value = false
  } catch (error) {
    console.error('强制关闭 Cursor 失败:', error)
    message.error('强制关闭 Cursor 失败')
  }
}

// 控制是否显示引导
const shouldShowTour = ref(false)

// 初始化
onMounted(async () => {
  
  try {
    // 加载设备信息
    await fetchUserInfo()
    await fetchMachineIds()
    await fetchCursorInfo()
    
    // 检查版本更新
    await checkUpdate()
    
    // 检查管理员权限
    await checkPrivileges()
    
    // 在所有检查完成后，如果没有模态框显示，则显示引导
    setTimeout(() => {
      if (!showUpdateModal.value && !showAdminPrivilegeModal.value && !showCursorRunningModal.value) {
        shouldShowTour.value = true
      }
    }, 1000)
  } catch (error) {
    console.error('初始化失败:', error)
    message.error('加载数据失败')
  }
})

// 监听模态框状态变化，如果有模态框显示，则隐藏引导
watch([showUpdateModal, showAdminPrivilegeModal, showCursorRunningModal, showCCStatusModal], 
  ([updateModal, adminModal, cursorModal, ccModal]) => {
    if (updateModal || adminModal || cursorModal || ccModal) {
      shouldShowTour.value = false
    } else {
      // 如果所有模态框都关闭了，并且是首次访问，则显示引导
      const hasTourShown = localStorage.getItem('dashboard_tour_shown')
      if (!hasTourShown || hasTourShown === 'false') {
        shouldShowTour.value = true
      }
    }
  }
)
</script>

<template>
  <n-space vertical size="large" class="dashboard-container">
    <!-- 使用网格布局排列卡片 -->
    <n-grid :cols="2" :x-gap="16" :y-gap="16">
      <!-- 用户信息卡片 -->
      <n-grid-item>
        <UserInfoCard :device-info="deviceInfo" :loading="loading" />
      </n-grid-item>
      
      <!-- 使用统计卡片 -->
      <n-grid-item>
        <UsageStatsCard :device-info="deviceInfo" :loading="loading" />
      </n-grid-item>
      
      <!-- 快捷操作卡片 -->
      <n-grid-item :span="2">
        <QuickActionsCard 
          :device-info="deviceInfo" 
          @machine-code-change="handleMachineCodeChange"
          @account-switch="handleAccountSwitch"
          @quick-change="handleQuickChange"
        />
      </n-grid-item>
    </n-grid>

    <!-- 模态框 -->
    <UpdateModal 
      :show="showUpdateModal" 
      :version-info="versionInfo" 
      @update:show="showUpdateModal = $event"
      @download="handleDownload" 
      @later="handleLater" 
    />

    <CursorRunningModal 
      :show="showCursorRunningModal" 
      :pending-action="pendingForceKillAction" 
      @update:show="showCursorRunningModal = $event"
      @force-kill="handleForceKill"
    />

    <AdminPrivilegeModal 
      :show="showAdminPrivilegeModal" 
      @update:show="showAdminPrivilegeModal = $event"
      @exit="handleExit" 
    />

    <CCStatusModal 
      :show="showCCStatusModal" 
      :original-action="originalActionBeforeHook" 
      @update:show="showCCStatusModal = $event"
    />

    <!-- 添加新的模态窗口 -->
    <InsufficientCreditsModal 
      :show="showInsufficientCreditsModal" 
      :user-credits="userCredits"
      @update:show="showInsufficientCreditsModal = $event"
      @activate-success="handleActivateSuccess"
    />

    <!-- 引导组件 - 只在没有模态框显示时显示 -->
    <DashboardTour v-if="shouldShowTour" />
  </n-space>
</template>

<style scoped>
/* 添加样式确保 grid 项目高度一致 */
.n-grid {
  grid-auto-rows: 1fr;
}

.n-grid-item {
  min-height: 0;
}

/* 限制容器高度，避免滚动条 */
.dashboard-container {
  max-height: calc(100vh - 80px);
  overflow: visible;
}
</style> 