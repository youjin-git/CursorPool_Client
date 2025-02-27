<script setup lang="ts">
import { onMounted } from 'vue'
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
    showCursorRunningModal.value = false
    
    // 强制关闭 Cursor 进程
    await killCursorProcess()
    await waitForCursorClose()
    
    // 根据挂起的操作类型执行相应的操作
    if (pendingForceKillAction.value) {
      const actionType = pendingForceKillAction.value.type
      
      if (actionType === 'machine') {
        await handleMachineCodeChange()
      } else if (actionType === 'account') {
        await handleAccountSwitch()
      } else if (actionType === 'quick') {
        await handleQuickChange()
      } else if (actionType === 'hook') {
        const originalAction = pendingForceKillAction.value.params?.originalAction
        await handleApplyHook(originalAction)
      }
      
      // 清除挂起的操作
      pendingForceKillAction.value = null
    }
  } catch (error) {
    console.error('强制关闭失败:', error)
    message.error('强制关闭失败')
  }
}

// 在组件挂载时获取所有信息
onMounted(async () => {
  console.log('DashboardView mounted')
  
  try {
    // 检查是否需要强制刷新数据
    const needRefresh = localStorage.getItem('need_refresh_dashboard')
    if (!needRefresh && (deviceInfo.value.userInfo || deviceInfo.value.cursorInfo.userInfo)) {
      return
    }
    // 清除刷新标记
    localStorage.removeItem('need_refresh_dashboard')

    loading.value = true
    // 按顺序执行
    await fetchUserInfo()
    await fetchMachineIds()
    await fetchCursorInfo()
    
    await checkPrivileges()
    await checkUpdate()
  } catch (error) {
    console.error('获取信息失败:', error)
    message.error('获取信息失败')
  } finally {
    loading.value = false
  }

  // 添加事件监听器
  window.addEventListener('refresh_dashboard_data', async () => {
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
  })
  
  // 调试高度问题
  const container = document.querySelector('.n-layout-scroll-container')
  console.log('Container height:', container?.clientHeight)
  console.log('Container scroll height:', container?.scrollHeight)
  console.log('Window height:', window.innerHeight)
  console.log('Body height:', document.body.clientHeight)
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

    <CCStatusModal 
      :show="showCCStatusModal" 
      :original-action="originalActionBeforeHook" 
      @update:show="showCCStatusModal = $event"
    />
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