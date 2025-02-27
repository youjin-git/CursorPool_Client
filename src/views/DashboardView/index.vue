<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useMessage } from 'naive-ui'
import { NGrid, NGridItem, NSpace } from 'naive-ui'
import { killCursorProcess, waitForCursorClose } from '@/api'

import UserInfoCard from './components/UserInfoCard.vue'
import UsageStatsCard from './components/UsageStatsCard.vue'
import QuickActionsCard from './components/QuickActionsCard.vue'
import UpdateModal from './components/UpdateModal.vue'
import CursorRunningModal from './components/CursorRunningModal.vue'
import AdminPrivilegeModal from './components/AdminPrivilegeModal.vue'
import CCStatusModal from './components/CCStatusModal.vue'
import DashboardTour from './components/DashboardTour.vue'

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
  handleAccountSwitch,
  handleQuickChange
} = useAccountActions()

// 导入注入相关方法
const { handleApplyHook } = useHookActions()

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
  console.log('DashboardView mounted')
  
  try {
    // 加载设备信息
    await fetchUserInfo()
    await fetchMachineIds()
    await fetchCursorInfo()
    
    // 检查版本更新
    await checkUpdate()
    
    // 检查管理员权限
    await checkPrivileges()
    
    // 调试信息
    const container = document.querySelector('.dashboard-container')
    console.log('Container height:', container?.clientHeight)
    console.log('Container scroll height:', container?.scrollHeight)
    console.log('Window height:', window.innerHeight)
    console.log('Body height:', document.body.clientHeight)
    
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