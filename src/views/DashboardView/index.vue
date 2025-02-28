<script setup lang="ts">
import { onMounted, ref, watch, onUnmounted } from 'vue'
import { useMessage } from 'naive-ui'
import { NGrid, NGridItem, NSpace } from 'naive-ui'
import { 
  killCursorProcess, 
  waitForCursorClose,
} from '@/api'

import UserInfoCard from './components/UserInfoCard.vue'
import UsageStatsCard from './components/UsageStatsCard.vue'
import QuickActionsCard from './components/QuickActionsCard.vue'
import UpdateModal from './components/UpdateModal.vue'
import CursorRunningModal from './components/CursorRunningModal.vue'
import AdminPrivilegeModal from './components/AdminPrivilegeModal.vue'
import DashboardTour from './components/DashboardTour.vue'
import InsufficientCreditsModal from './components/InsufficientCreditsModal.vue'
import DisclaimerModal from './components/DisclaimerModal.vue'

import { useDashboardState } from './composables/useDashboardState'
import { useDeviceInfo } from './composables/useDeviceInfo'
import { useVersionCheck } from './composables/useVersionCheck'
import { usePrivilegeCheck } from './composables/usePrivilegeCheck'
import { useAccountActions } from './composables/useAccountActions'

const message = useMessage()

// 导入状态和方法
const { 
  loading, 
  showUpdateModal, 
  showCursorRunningModal,
  showAdminPrivilegeModal,
  pendingForceKillAction,
  versionInfo
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
  handleAccountSwitch,
  handleQuickChange,
  showInsufficientCreditsModal,
  userCredits,
  handleActivateSuccess,
} = useAccountActions()

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
      }
      
      // 重置待处理操作
      pendingForceKillAction.value = null
    }
    message.success('cursor会自动重启')
    // 关闭模态框
    showCursorRunningModal.value = false
  } catch (error) {
    console.error('强制关闭 Cursor 失败:', error)
    message.error('强制关闭 Cursor 失败')
  }
}

// 控制是否显示引导
const shouldShowTour = ref(false)
// 控制是否显示免责声明
const showDisclaimerModal = ref(false)

const handleContinueOriginalAction = (event: Event) => {
  const customEvent = event as CustomEvent;
  if (customEvent.detail && customEvent.detail.actionType) {
    const actionType = customEvent.detail.actionType;
    if (actionType === 'account') {
      handleAccountSwitch();
    } else if (actionType === 'quick') {
      handleQuickChange();
    } else if (actionType === 'machine') {
      handleMachineCodeChange();
    }
  }
};

// 创建刷新函数
const refreshDashboardData = async () => {
  try {
    loading.value = true
    await fetchUserInfo()
    await fetchMachineIds()
    await fetchCursorInfo()
  } catch (error) {
    console.error('刷新数据失败:', error)
    message.error('刷新数据失败')
  } finally {
    loading.value = false
  }
}

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
    
    // 在所有检查完成后，检查是否需要显示引导或免责声明
    setTimeout(() => {
      const apiKey = localStorage.getItem('apiKey')
      const disclaimerAccepted = localStorage.getItem('disclaimer_accepted')
      
      // 如果已登录且没有其他模态框显示
      if (apiKey && 
          !showUpdateModal.value && 
          !showAdminPrivilegeModal.value && 
          !showCursorRunningModal.value
      ) {
        // 如果未接受免责声明，显示免责声明
        if (disclaimerAccepted !== 'true') {
          showDisclaimerModal.value = true
        } 
        // 否则如果未显示过引导，显示引导
        else if (!localStorage.getItem('dashboard_tour_shown')) {
          shouldShowTour.value = true
        }
      }
    }, 1000)

    // 注册事件监听器（使用具名函数）
    window.addEventListener('continue_original_action', handleContinueOriginalAction);
    
    // 使用新的刷新函数
    window.addEventListener('refresh_dashboard_data', refreshDashboardData)
  } catch (error) {
    console.error('初始化失败:', error)
    message.error('加载数据失败')
  }
})

// 监听模态框状态变化，如果有模态框显示，则隐藏引导
watch([showUpdateModal, showAdminPrivilegeModal, showCursorRunningModal], 
  ([updateModal, adminModal, cursorModal]) => {
    if (updateModal || adminModal || cursorModal) {
      shouldShowTour.value = false
    } else {
      // 如果所有模态框都关闭了，并且是首次访问，则显示引导
      const apiKey = localStorage.getItem('apiKey')
      const hasTourShown = localStorage.getItem('dashboard_tour_shown')
      if (apiKey && (!hasTourShown || hasTourShown === 'false')) {
        shouldShowTour.value = true
      }
    }
  }
)

// 在组件卸载时移除事件监听器
onUnmounted(() => {
  // 移除事件监听器
  window.removeEventListener('refresh_dashboard_data', refreshDashboardData)
  window.removeEventListener('continue_original_action', handleContinueOriginalAction)
})
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

    <!-- 添加新的模态窗口 -->
    <InsufficientCreditsModal 
      :show="showInsufficientCreditsModal" 
      :user-credits="userCredits"
      @update:show="showInsufficientCreditsModal = $event"
      @activate-success="handleActivateSuccess"
    />

    <!-- 添加免责声明模态框 -->
    <DisclaimerModal
      v-model:show="showDisclaimerModal"
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