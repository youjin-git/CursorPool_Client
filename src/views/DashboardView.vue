<script setup lang="ts">
import { NCard, NSpace, NButton, NProgress, NNumberAnimation, NGrid, NGridItem, NTag, NDivider, NModal, NIcon } from 'naive-ui'
import { ref, onMounted, computed } from 'vue'
import { useI18n } from '../locales'
import { useMessage } from 'naive-ui'
import { 
    getUserInfo, 
    resetMachineId, 
    switchAccount, 
    getMachineIds, 
    getUsage, 
    getAccount, 
    getVersion, 
    checkCursorRunning,
    killCursorProcess,
    waitForCursorClose,
    checkAdminPrivileges,
    checkUpdateDisabled,
    checkHookStatus,
    checkIsWindows
} from '@/api'
import type { UserInfo, CursorUserInfo, CursorUsageInfo, VersionInfo } from '@/api/types'
import { addHistoryRecord } from '../utils/history'
import { version } from '../../package.json'
import { WarningOutlined } from '@vicons/antd'
import { Window } from '@tauri-apps/api/window'
import { open } from '@tauri-apps/plugin-shell'

const LOCAL_VERSION = version

// 版本检查的时间间隔（毫秒）
const VERSION_CHECK_INTERVAL = 3 * 60 * 60 * 1000 // 3小时

interface DeviceInfoState {
  machineCode: string
  currentAccount: string
  cursorToken: string
  userInfo: UserInfo | null
  cursorInfo: {
    userInfo: CursorUserInfo | null
    usage: CursorUsageInfo | null
  }
  hookStatus: boolean | null
}

// 格式化日期
const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
}

const deviceInfo = ref<DeviceInfoState>({
  machineCode: '',
  currentAccount: '',
  cursorToken: '',
  userInfo: null,
  cursorInfo: {
    userInfo: null,
    usage: null
  },
  hookStatus: null
})

const loading = ref(true)

const message = useMessage()
const { i18n } = useI18n()

// 计算使用量百分比
const getUsagePercentage = (used: number, total: number) => {
  if (!total) return 0
  return Math.min(100, Math.round((used / total) * 100))
}

// 会员等级映射
const levelMap: Record<number, { name: string; type: 'default' | 'info' | 'success' | 'warning' | 'error' }> = {
  1: { name: i18n.value.dashboard.memberLevel[1], type: 'default' },
  2: { name: i18n.value.dashboard.memberLevel[2], type: 'info' },
  3: { name: i18n.value.dashboard.memberLevel[3], type: 'success' },
  4: { name: i18n.value.dashboard.memberLevel[4], type: 'warning' },
  5: { name: i18n.value.dashboard.memberLevel[5], type: 'error' }
}

// 普通账户使用量百分比
const accountUsagePercentage = computed(() => {
  if (!deviceInfo.value.userInfo?.totalCount) return 0
  return getUsagePercentage(
    deviceInfo.value.userInfo.usedCount,
    deviceInfo.value.userInfo.totalCount
  )
})

// Cursor高级模型使用量百分比
const cursorGpt4Percentage = computed(() => {
  const usage = deviceInfo.value.cursorInfo.usage?.['gpt-4']
  if (!usage) return 0
  return getUsagePercentage(usage.numRequests, usage.maxRequestUsage || 0)
})

// Cursor普通模型使用量百分比
const cursorGpt35Percentage = computed(() => {
  const usage = deviceInfo.value.cursorInfo.usage?.['gpt-3.5-turbo']
  if (!usage) return 0
  if (!usage.maxRequestUsage) return 100
  return getUsagePercentage(usage.numRequests, usage.maxRequestUsage)
})

// 获取用户信息
const fetchUserInfo = async () => {
  try {
    const apiKey = localStorage.getItem('apiKey')
    if (!apiKey) {
      throw new Error('未找到 API Key')
    }
    const info = await getUserInfo(apiKey)
    deviceInfo.value.userInfo = info
  } catch (error) {
    localStorage.removeItem('apiKey')
    console.error('获取用户信息失败:', error)
  }
}

// 获取机器码
const fetchMachineIds = async () => {
  try {
    const result = await getMachineIds()

    deviceInfo.value.machineCode = result.machineId
    deviceInfo.value.currentAccount = result.currentAccount
    deviceInfo.value.cursorToken = result.cursorToken
    
    // 获取 Hook 状态
    deviceInfo.value.hookStatus = await checkHookStatus()
  } catch (error) {
    console.error('获取机器码失败:', error)
  }
}

// 获取 Cursor 账户信息
async function fetchCursorInfo() {
  try {
    const token = deviceInfo.value.cursorToken
    if (!token) {
      console.error('未找到 Cursor Token')
      return
    }

    const usageData = await getUsage(token)
    
    deviceInfo.value.cursorInfo = {
      userInfo: {
        email: deviceInfo.value.currentAccount,
        email_verified: true,
        name: deviceInfo.value.currentAccount.split('@')[0],
        sub: '',
        updatedAt: new Date().toISOString(),
        picture: null
      },
      usage: usageData
    }
  } catch (error) {
    console.error('获取 Cursor 账户信息失败:', error)
  } finally {
    loading.value = false
  }
}

// 添加新的 ref
const showCursorRunningModal = ref(false)
const pendingForceKillAction = ref<{
  type: 'machine' | 'account' | 'quick',
  params?: any
} | null>(null)

// 添加 CC 状态检查模态框
const showCCStatusModal = ref(false)

// 修改机器码更换处理函数
const handleMachineCodeChange = async (force_kill: boolean = false) => {
  // 先检查 CC 状态
  if (!deviceInfo.value.hookStatus) {
    showCCStatusModal.value = true
    return
  }

  try {
    await resetMachineId(force_kill)
    message.success(i18n.value.dashboard.machineChangeSuccess)
    addHistoryRecord(
      '机器码修改',
      `修改机器码: ${deviceInfo.value.machineCode}`
    )
    await fetchMachineIds()
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

// 添加新的 ref
const showUnusedCreditsModal = ref(false)
const unusedCredits = ref(0)
const pendingAction = ref<'account' | 'quick' | null>(null)

// 检查未使用的积分
const checkUnusedCredits = () => {
  // 先检查是否有 usage 数据
  if (!deviceInfo.value.cursorInfo.usage) {
    return false
  }

  const gpt4Usage = deviceInfo.value.cursorInfo.usage['gpt-4']
  if (gpt4Usage && gpt4Usage.maxRequestUsage) {
    const remaining = gpt4Usage.maxRequestUsage - gpt4Usage.numRequests
    if (remaining > 140) {
      unusedCredits.value = remaining
      showUnusedCreditsModal.value = true
      return true
    }
  }
  return false
}

// 修改账户切换处理函数
const handleAccountSwitch = async () => {
  // 先检查 CC 状态
  if (!deviceInfo.value.hookStatus) {
    showCCStatusModal.value = true
    return
  }

  // 再检查未使用的积分
  if (checkUnusedCredits()) {
    pendingAction.value = 'account'
    return
  }

  // 最后检查 Cursor 是否在运行
  const isRunning = await checkCursorRunning()
  if (isRunning) {
    showCursorRunningModal.value = true
    pendingForceKillAction.value = { type: 'account' }
    return
  }

  await executeAccountSwitch()
}

// 修改一键切换处理函数
const handleQuickChange = async () => {
  // 先检查 CC 状态
  if (!deviceInfo.value.hookStatus) {
    showCCStatusModal.value = true
    return
  }

  // 再检查未使用的积分
  if (checkUnusedCredits()) {
    pendingAction.value = 'quick'
    return
  }

  // 最后检查 Cursor 是否在运行
  const isRunning = await checkCursorRunning()
  if (isRunning) {
    showCursorRunningModal.value = true
    pendingForceKillAction.value = { type: 'quick' }
    return
  }

  await executeQuickChange()
}

// 修改确认切换函数
const handleConfirmSwitch = async () => {
  showUnusedCreditsModal.value = false
  if (pendingAction.value === 'account') {
    // 检查 Cursor 是否在运行
    const isRunning = await checkCursorRunning()
    if (isRunning) {
      showCursorRunningModal.value = true
      pendingForceKillAction.value = { type: 'account' }
      return
    }
    await executeAccountSwitch()
  } else if (pendingAction.value === 'quick') {
    // 检查 Cursor 是否在运行
    const isRunning = await checkCursorRunning()
    if (isRunning) {
      showCursorRunningModal.value = true
      pendingForceKillAction.value = { type: 'quick' }
      return
    }
    await executeQuickChange()
  }
  pendingAction.value = null
}

const handleCancelSwitch = () => {
  showUnusedCreditsModal.value = false
  pendingAction.value = null
}

// 修改账户切换执行函数
const executeAccountSwitch = async (force_kill: boolean = false) => {
  try {
    const apiKey = localStorage.getItem('apiKey')
    if (!apiKey) {
      message.error(i18n.value.message.pleaseInputEmail)
      return
    }

    // 先检查 Cursor 是否在运行
    const isRunning = await checkCursorRunning()
    if (isRunning && !force_kill) {
      showCursorRunningModal.value = true
      pendingForceKillAction.value = { type: 'account' }
      return
    }

    // 获取账号信息并执行实际的切换
    const accountInfo = await getAccount(apiKey)
    
    if (!accountInfo.email || !accountInfo.token) {
      message.error(i18n.value.dashboard.accountChangeFailed)
      return
    }
    
    await switchAccount(accountInfo.email, accountInfo.token, force_kill)
    message.success(i18n.value.dashboard.accountChangeSuccess)
    addHistoryRecord(
      '账户切换',
      `切换到账户: ${accountInfo.email} 扣除50积分`
    )
    await Promise.all([
      fetchUserInfo(),
      fetchMachineIds(),
      fetchCursorInfo()
    ])
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    if (errorMsg === 'Cursor进程正在运行, 请先关闭Cursor') {
      showCursorRunningModal.value = true
      pendingForceKillAction.value = { type: 'account' }
      return
    }
    console.error('切换账户失败:', error)
    message.error(i18n.value.common.copyFailed)
  }
}

// 修改一键切换执行函数
const executeQuickChange = async (force_kill: boolean = false) => {
  try {
    await executeAccountSwitch(force_kill)
    await handleMachineCodeChange(force_kill)
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

// 修改 killCursorProcess 函数
const handleKillCursorProcess = async () => {
    try {
        await killCursorProcess()
        // 开始轮询检查进程状态
        return await waitForCursorClose()
    } catch (error) {
        throw new Error('关闭 Cursor 失败')
    }
}

// 修改强制关闭处理函数
const handleForceKill = async () => {
    showCursorRunningModal.value = false
    if (!pendingForceKillAction.value) return

    try {
        loading.value = true
        message.loading('正在关闭 Cursor...', { duration: 0 })
        
        await handleKillCursorProcess()
        message.destroyAll() // 清除 loading 消息
        
        // 根据类型执行相应操作, 但不再传入 force_kill 参数
        switch (pendingForceKillAction.value.type) {
            case 'machine':
                await handleMachineCodeChange()
                message.success(i18n.value.common.copySuccess + ', 正在清理进程, 稍后将自动重启Cursor')
                break
            case 'account':
                await executeAccountSwitch()
                message.success(i18n.value.dashboard.accountChangeSuccess + ', 正在清理进程, 稍后将自动重启Cursor')
                break
            case 'quick':
                await executeQuickChange()
                message.success(i18n.value.common.copySuccess + ', 正在清理进程, 稍后将自动重启Cursor')
                break
        }
    } catch (error) {
        message.destroyAll() // 清除 loading 消息
        message.error('操作失败: ' + (error instanceof Error ? error.message : String(error)))
    } finally {
        loading.value = false
        pendingForceKillAction.value = null
    }
}

const copyText = (text: string) => {
  if (!text) return
  navigator.clipboard.writeText(text).then(() => {
    message.success(i18n.value.common.copySuccess)
  }).catch(() => {
    message.error(i18n.value.common.copyFailed)
  })
}

// 添加版本检查相关的状态
const showUpdateModal = ref(false)
const versionInfo = ref<VersionInfo | null>(null)

// 版本比较函数
const compareVersions = (v1: string, v2: string) => {
  const parts1 = v1.split('.').map(Number)
  const parts2 = v2.split('.').map(Number)
  
  for (let i = 0; i < 3; i++) {
    if (parts1[i] > parts2[i]) return 1
    if (parts1[i] < parts2[i]) return -1
  }
  return 0
}

// 检查版本更新
const checkUpdate = async () => {
  try {
    const apiKey = localStorage.getItem('apiKey')
    if (!apiKey) return
    
    // 检查上次更新提示的时间
    const lastCheckTime = localStorage.getItem('last_version_check_time')
    const now = Date.now()
    
    if (lastCheckTime) {
      const timeDiff = now - parseInt(lastCheckTime)
      if (timeDiff < VERSION_CHECK_INTERVAL) {
        return // 如果间隔小于3小时, 不进行检查
      }
    }
    
    const remoteVersionInfo = await getVersion()
    versionInfo.value = remoteVersionInfo
    
    if (compareVersions(LOCAL_VERSION, remoteVersionInfo.version) < 0) {
      showUpdateModal.value = true
      // 只有在非强制更新时才更新检查时间
      if (!remoteVersionInfo.forceUpdate) {
        localStorage.setItem('last_version_check_time', now.toString())
      }
    }
  } catch (error) {
    console.error('检查更新失败:', error)
  }
}

// 处理下载更新
const handleDownload = () => {
  if (versionInfo.value?.downloadUrl) {
    window.open(versionInfo.value.downloadUrl, '_blank')
  }
}

// 处理稍后更新
const handleLater = () => {
  showUpdateModal.value = false
  // 记录关闭时间
  localStorage.setItem('last_version_check_time', Date.now().toString())
}

// 添加新的 ref
const showAdminPrivilegeModal = ref(false)

// 检查管理员权限
const checkPrivileges = async () => {
  try {
    const isAdmin = await checkAdminPrivileges();
    if (!isAdmin) {
      // 如果不是管理员，再检查是否是 Windows 平台
      const isWindows = await checkIsWindows();
      if (isWindows) {
        showAdminPrivilegeModal.value = true;
      }
    }
  } catch (error) {
    console.error('检查管理员权限失败:', error);
    message.error('检查管理员权限失败');
  }
}

// 退出程序
const handleExit = async () => {
  const appWindow = new Window('main')
  await appWindow.close()
}

// 添加更新状态 ref
const updateDisabled = ref(false)

// 在组件挂载时获取所有信息
onMounted(async () => {
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
    
    // 检查更新状态
    updateDisabled.value = await checkUpdateDisabled()
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
})

// 修改按钮点击处理函数
const handleMachineCodeClick = () => handleMachineCodeChange(false)

// 添加系统检测和链接处理
const handleHistoryDownload = async () => {
  try {
    const url = 'https://downloader-cursor.deno.dev/'
    await open(url)
  } catch (error) {
    console.error('打开链接失败:', error)
    message.error('打开链接失败')
  }
}
</script>

<template>
  <n-space vertical size="large">
    <n-grid :cols="2" :x-gap="24" style="display: grid; grid-template-columns: repeat(2, 1fr);">
      <!-- 用户信息卡片 -->
      <n-grid-item style="display: grid;">
        <n-card :title="i18n.dashboard.userInfo" style="height: 100%; user-select: none;">
          <n-space vertical>
            <n-space vertical :size="12" style="user-select: none;">
              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">{{ i18n.dashboard.username }}</span>
                <n-space :size="4" align="center">
                  <span 
                    style="font-size: 14px; cursor: pointer;" 
                    @click="deviceInfo.userInfo?.username && copyText(deviceInfo.userInfo.username)"
                  >{{ deviceInfo.userInfo?.username }}</span>
                  <n-tag :type="levelMap[deviceInfo.userInfo?.level || 1].type" size="tiny" style="transform: scale(0.9)">
                    {{ levelMap[deviceInfo.userInfo?.level || 1].name }}
                  </n-tag>
                </n-space>
              </n-space>

              <n-divider style="margin: 0" />

              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">{{ i18n.dashboard.email }}</span>
                <n-space :size="4" align="center">
                  <span 
                    style="font-size: 14px; cursor: pointer;" 
                    @click="deviceInfo.cursorInfo.userInfo?.email && copyText(deviceInfo.cursorInfo.userInfo?.email)"
                  >{{ deviceInfo.cursorInfo.userInfo?.email || '未绑定' }}</span>
                  <n-tag :type="deviceInfo.cursorInfo.userInfo?.email_verified ? 'success' : 'warning'" size="tiny" style="transform: scale(0.9)">
                    {{ deviceInfo.cursorInfo.userInfo?.email_verified ? i18n.systemControl.clientVerified : i18n.systemControl.clientUnverified }}
                  </n-tag>
                </n-space>
              </n-space>
              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">{{ i18n.dashboard.ccStatus }}</span>
                <n-tag :type="deviceInfo.hookStatus === true ? 'success' : 'error'" size="small">
                  {{ deviceInfo.hookStatus === true ? i18n.systemControl.hookApplied : i18n.systemControl.hookNotApplied }}
                </n-tag>
              </n-space>
              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">{{ i18n.dashboard.registerTime }}</span>
                <span 
                  style="font-size: 14px; cursor: pointer;" 
                  @click="copyText(deviceInfo.cursorInfo.usage?.startOfMonth ? formatDate(deviceInfo.cursorInfo.usage.startOfMonth) : '')"
                >{{ deviceInfo.cursorInfo.usage?.startOfMonth ? formatDate(deviceInfo.cursorInfo.usage.startOfMonth) : '未知' }}</span>
              </n-space>
              <span 
                style="font-size: 12px; color: #999; word-break: break-all; text-align: center; cursor: pointer;" 
                @click="copyText(deviceInfo.machineCode)"
              >{{ deviceInfo.machineCode }}</span>
            </n-space>
          </n-space>
        </n-card>
      </n-grid-item>

      <!-- 使用统计卡片 -->
      <n-grid-item style="display: grid;">
        <n-card :title="i18n.dashboard.usageStats" style="height: 100%; user-select: none;">
          <n-space vertical :size="24" style="height: 100%; justify-content: space-around;">
            <!-- 账户使用统计 -->
            <n-space vertical :size="8">
              <n-space justify="space-between">
                <span>{{ i18n.dashboard.cpUsage }}</span>
                <n-space :size="0">
                  <n-number-animation 
                    :from="0" 
                    :to="(deviceInfo.userInfo?.usedCount || 0) * 50"0
                    :duration="1000"
                  />
                  <span>/{{ (deviceInfo.userInfo?.totalCount || 0) * 50 }}</span>
                </n-space>
              </n-space>
              <n-progress
                type="line"
                status="success"
                :percentage="accountUsagePercentage"
                :show-indicator="false"
                :height="12"
                :border-radius="6"
                :processing="loading"
              />
            </n-space>

            <!-- Cursor GPT-4 使用统计 -->
            <n-space vertical :size="8">
              <n-space justify="space-between">
                <span>{{ i18n.dashboard.advancedModelUsage }}</span>
                <n-space v-if="deviceInfo.cursorInfo.usage" :size="0">
                  <n-number-animation 
                    :from="0"
                    :to="deviceInfo.cursorInfo.usage['gpt-4']?.numRequests || 0"
                    :duration="1000"
                  />
                  <span>/{{ deviceInfo.cursorInfo.usage['gpt-4']?.maxRequestUsage || 0 }}</span>
                </n-space>
                <span v-else>{{ i18n.dashboard.cannotGetUsage }}</span>
              </n-space>
              <n-progress
                type="line"
                status="success"
                :percentage="cursorGpt4Percentage"
                :show-indicator="false"
                :height="12"
                :border-radius="6"
                :processing="loading"
              />
            </n-space>

            <!-- Cursor GPT-3.5 使用统计 -->
            <n-space vertical :size="8">
              <n-space justify="space-between">
                <span>{{ i18n.dashboard.basicModelUsage }}</span>
                <n-space v-if="deviceInfo.cursorInfo.usage" :size="0">
                  <n-number-animation 
                    :from="0" 
                    :to="deviceInfo.cursorInfo.usage['gpt-3.5-turbo']?.numRequests || 0"
                    :duration="1000"
                  />
                  <span v-if="deviceInfo.cursorInfo.usage['gpt-3.5-turbo']?.maxRequestUsage">
                    /{{ deviceInfo.cursorInfo.usage['gpt-3.5-turbo']?.maxRequestUsage }}
                  </span>
                  <span v-else>/{{ i18n.dashboard.unlimited }}</span>
                </n-space>
                <span v-else>{{ i18n.dashboard.cannotGetUsage }}</span>
              </n-space>
              <n-progress
                type="line"
                status="success"
                :percentage="cursorGpt35Percentage"
                :show-indicator="false"
                :height="12"
                :border-radius="6"
                :processing="loading"
              />
            </n-space>

          </n-space>
        </n-card>
      </n-grid-item>
    </n-grid>

    <!-- 快捷操作卡片 -->
    <n-card :title="i18n.dashboard.quickActions" style="user-select: none;">
      <n-space vertical>
        <n-space justify="space-around">
          <n-button type="primary" @click="handleQuickChange" :disabled="!deviceInfo.userInfo">
            {{ i18n.dashboard.quickChange }}
          </n-button>
          <n-button type="primary" @click="handleAccountSwitch" :disabled="!deviceInfo.userInfo">
            {{ i18n.dashboard.changeAccount }}
          </n-button>
          <n-button type="primary" @click="handleMachineCodeClick">
            {{ i18n.dashboard.changeMachineCode }}
          </n-button>
        </n-space>
      </n-space>
    </n-card>

    <!-- 添加更新模态框 -->
    <n-modal
      v-model:show="showUpdateModal"
      :mask-closable="!versionInfo?.forceUpdate"
      :closable="!versionInfo?.forceUpdate"
      preset="card"
      style="width: 500px"
      :title="i18n.dashboard.newVersionAvailable"
    >
      <n-space vertical>
        <div>{{ i18n.dashboard.currentVersion }}: {{ LOCAL_VERSION }}</div>
        <div>{{ i18n.dashboard.newVersion }}: {{ versionInfo?.version }}</div>
        <n-divider />
        <div style="white-space: pre-line">{{ versionInfo?.changeLog }}</div>
        <n-space justify="end">
          <n-button
            v-if="!versionInfo?.forceUpdate"
            @click="handleLater"
          >
            {{ i18n.dashboard.later }}
          </n-button>
          <n-button
            type="primary"
            @click="handleDownload"
          >
            {{ i18n.dashboard.downloadNow }}
          </n-button>
        </n-space>
      </n-space>
    </n-modal>

    <n-modal
      v-model:show="showUnusedCreditsModal"
      preset="dialog"
      title="使用提醒"
      :closable="true"
      :mask-closable="false"
    >
      <template #default>
        <p>您还有 {{ unusedCredits }} 次高级模型使用次数未使用</p>
        <p style="margin-top: 12px; color: #666;">
          {{ pendingAction === 'quick' ? '一键切换将扣除50积分' : '切换账号将扣除50积分' }}，确定要继续吗？
        </p>
      </template>
      <template #action>
        <n-space justify="end">
          <n-button @click="handleCancelSwitch">
            取消
          </n-button>
          <n-button type="primary" @click="handleConfirmSwitch">
            确认切换
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 添加 Cursor 运行提醒模态框 -->
    <n-modal
      v-model:show="showCursorRunningModal"
      preset="dialog"
      title="Cursor 正在运行"
      :closable="true"
      :mask-closable="false"
    >
      <template #default>
        检测到 Cursor 正在运行, 请保存尚未更改的项目再继续操作!
      </template>
      <template #action>
        <n-space justify="end">
          <n-button type="warning" @click="handleForceKill">
            我已保存, 强制关闭
          </n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 添加管理员权限提示模态框 -->
    <n-modal
      v-model:show="showAdminPrivilegeModal"
      preset="dialog"
      title="需要管理员权限"
      :closable="false"
      :mask-closable="false"
      style="width: 500px"
    >
      <template #header>
        <n-space align="center">
          <n-icon size="24" color="#f0a020">
            <warning-outlined />
          </n-icon>
          <span>需要管理员权限</span>
        </n-space>
      </template>
      <div style="margin: 24px 0;">
        <p>本程序需要管理员权限才能正常运行。</p>
        <p style="margin-top: 12px; color: #999;">
          请右键点击程序图标,选择"以管理员身份运行"后重新启动程序。
        </p>
      </div>
      <template #action>
        <n-button type="error" @click="handleExit" block>
          退出程序
        </n-button>
      </template>
    </n-modal>

    <!-- 添加 CC 状态检查模态框 -->
    <n-modal
      v-model:show="showCCStatusModal"
      preset="dialog"
      title="CC 客户端未注入"
      :closable="true"
      :mask-closable="true"
    >
      <template #default>
        <p>检测到 Cursor 客户端未注入，在此状态下进行操作可能会导致：</p>
        <ul style="margin: 12px 0; padding-left: 20px; color: #ff4d4f;">
          <li>账户更换失败</li>
          <li>积分异常扣除</li>
        </ul>
        <p>请确保 Cursor 客户端正常注入后再进行操作</p>
      </template>
      <template #action>
        <n-button type="primary" @click="showCCStatusModal = false">
          我知道了
        </n-button>
      </template>
    </n-modal>

    <n-space justify="center" style="margin-top: 24px;">
      <n-button
        text
        @click="handleHistoryDownload"
        style="font-size: 12px;"
      >
        {{ i18n.dashboard.cursorHistoryDownload }}
      </n-button>
    </n-space>
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
</style>
