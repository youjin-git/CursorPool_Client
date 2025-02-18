<script setup lang="ts">
import { NCard, NSpace, NButton, NProgress, NNumberAnimation, NGrid, NGridItem, NTag, NDivider, NModal } from 'naive-ui'
import { ref, onMounted, computed } from 'vue'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import { useMessage } from 'naive-ui'
import { getUserInfo, resetMachineIdOnly, switchAccount, getMachineIds, getUserInfoCursor, getUsage, getAccount, getVersion } from '@/api'
import type { Language } from '../locales'
import type { UserInfo, CursorUserInfo, CursorUsageInfo, VersionInfo } from '@/api/types'
import { addHistoryRecord } from '../utils/history'

// 本地版本号
const LOCAL_VERSION = '0.1.0'

// 版本检查的时间间隔（毫秒）
const VERSION_CHECK_INTERVAL = 3 * 60 * 60 * 1000 // 3小时

interface DeviceInfoState {
  machineCode: string
  currentAccount: string
  userInfo: UserInfo | null
  cursorInfo: {
    userInfo: CursorUserInfo | null
    usage: CursorUsageInfo | null
  }
}

// 格式化日期
const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
}

const deviceInfo = ref<DeviceInfoState>({
  machineCode: '',
  currentAccount: '',
  userInfo: null,
  cursorInfo: {
    userInfo: null,
    usage: null
  }
})

const loading = ref(true)

const message = useMessage()
const { currentLang } = useI18n()
const i18n = computed(() => messages[currentLang.value as Language])

// 计算剩余时间
const getRemainingTime = (expireTime: number) => {
  if (!expireTime) return '0分钟'
  const now = new Date().getTime()
  // 将秒级时间戳转换为毫秒级
  const expireTimeMs = expireTime * 1000
  const diff = expireTimeMs - now
  if (diff <= 0) return '已过期'
  
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  if (days > 0) {
    return `${days}天`
  }
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60))
  return `${hours}小时${minutes}分钟`
}

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
  return 100  // 始终返回 100%
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
    const apiKey = localStorage.getItem('api_key')
    if (!apiKey) {
      throw new Error('未找到 API Key')
    }
    const info = await getUserInfo(apiKey)
    deviceInfo.value.userInfo = info
  } catch (error) {
    console.error('获取用户信息失败:', error)
  }
}

// 获取机器码
const fetchMachineIds = async () => {
  try {
    const result = await getMachineIds()

    deviceInfo.value.machineCode = result.machine_id
    deviceInfo.value.currentAccount = result.current_account
  } catch (error) {
    console.error('获取机器码失败:', error)
  }
}

// 获取 Cursor 账户信息
async function fetchCursorInfo() {
  try {
    const userId = localStorage.getItem('cache.cursor.userId')
    const token = localStorage.getItem('cache.cursor.token')
    
    if (!userId || !token) {
      return
    }

    const userInfoData = await getUserInfoCursor(userId, token)
    const usageData = await getUsage(userId, token)
    
    deviceInfo.value.cursorInfo = {
      userInfo: userInfoData,
      usage: usageData
    }
  } catch (error) {
    console.error('获取 Cursor 账户信息失败:', error)
  } finally {
    loading.value = false
  }
}

// 处理机器码更换
const handleMachineCodeChange = async () => {
  try {
    await resetMachineIdOnly()
    message.success(i18n.value.dashboard.machineChangeSuccess)
    addHistoryRecord(
      '机器码修改',
      `修改机器码: ${deviceInfo.value.machineCode}`
    )
    await fetchMachineIds()
  } catch (error) {
    message.error(i18n.value.dashboard.machineChangeFailed)
  }
}

// 处理账户切换
const handleAccountSwitch = async () => {
  try {
    const apiKey = localStorage.getItem('api_key')
    if (!apiKey) {
      message.error(i18n.value.message.pleaseInputEmail)
      return
    }

    // 先获取账户信息
    const accountInfo = await getAccount(apiKey)
    
    if (!accountInfo.email || !accountInfo.token) {
      message.error(i18n.value.dashboard.accountChangeFailed)
      return
    }

    // 保存到本地存储
    localStorage.setItem('cache.cursor.userId', accountInfo.user_id)
    localStorage.setItem('cache.cursor.token', accountInfo.token)
    console.log(accountInfo.user_id);
    
    // 调用切换账户
    await switchAccount(accountInfo.email, accountInfo.token)

    message.success(i18n.value.dashboard.accountChangeSuccess)
    addHistoryRecord(
      '账户切换',
      `切换到账户: ${accountInfo.email}`
    )
    await Promise.all([
      fetchUserInfo(),
      fetchMachineIds(),
      fetchCursorInfo()
    ])
  } catch (error) {
    console.error('切换账户失败:', error)
    message.error(i18n.value.dashboard.accountChangeFailed)
  }
}

// 一键切换
const handleQuickChange = async () => {
  try {
    await handleAccountSwitch()
    await handleMachineCodeChange()
    message.success(i18n.value.dashboard.changeSuccess)
    addHistoryRecord(
      '一键切换',
      '完成账户和机器码的切换'
    )
  } catch (error) {
    message.error(i18n.value.dashboard.changeFailed)
  }
}

const copyText = (text: string) => {
  if (!text) return
  navigator.clipboard.writeText(text).then(() => {
    message.success('复制成功')
  }).catch(() => {
    message.error('复制失败')
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
    const apiKey = localStorage.getItem('api_key')
    if (!apiKey) return
    
    // 检查上次更新提示的时间
    const lastCheckTime = localStorage.getItem('last_version_check_time')
    const now = Date.now()
    
    if (lastCheckTime) {
      const timeDiff = now - parseInt(lastCheckTime)
      if (timeDiff < VERSION_CHECK_INTERVAL) {
        return // 如果间隔小于3小时，不进行检查
      }
    }
    
    const remoteVersionInfo = await getVersion()
    versionInfo.value = remoteVersionInfo
    
    if (compareVersions(LOCAL_VERSION, remoteVersionInfo.version) < 0) {
      showUpdateModal.value = true
      // 更新检查时间
      localStorage.setItem('last_version_check_time', now.toString())
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
    await Promise.all([
      fetchUserInfo(),
      fetchMachineIds(),
      fetchCursorInfo()
    ])
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
      await Promise.all([
        fetchUserInfo(),
        fetchMachineIds(),
        fetchCursorInfo()
      ])
    } catch (error) {
      console.error('刷新数据失败:', error)
      message.error('刷新数据失败')
    } finally {
      loading.value = false
    }
  })
})
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
                <span style="width: 70px">用户名</span>
                <n-space :size="4" align="center">
                  <span 
                    style="font-size: 14px; cursor: pointer;" 
                    @click="deviceInfo.userInfo?.username && copyText(deviceInfo.userInfo.username)"
                  >{{ deviceInfo.userInfo?.username }}</span>
                  <n-tag :type="deviceInfo.userInfo?.is_expired ? 'error' : 'success'" size="tiny" style="transform: scale(0.9)">
                    {{ deviceInfo.userInfo?.is_expired ? '已过期' : getRemainingTime(deviceInfo.userInfo?.expire_time || 0) }}
                  </n-tag>
                </n-space>
              </n-space>
              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">会员等级</span>
                <n-tag :type="levelMap[deviceInfo.userInfo?.level || 1].type" size="small">
                  {{ levelMap[deviceInfo.userInfo?.level || 1].name }}
                </n-tag>
              </n-space>

              <n-divider style="margin: 0" />

              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">CC邮箱</span>
                <n-space :size="4" align="center">
                  <span 
                    style="font-size: 14px; cursor: pointer;" 
                    @click="deviceInfo.cursorInfo.userInfo?.email && copyText(deviceInfo.cursorInfo.userInfo?.email)"
                  >{{ deviceInfo.cursorInfo.userInfo?.email || '未绑定' }}</span>
                  <n-tag v-if="deviceInfo.cursorInfo.userInfo?.email" :type="deviceInfo.cursorInfo.userInfo?.email_verified ? 'success' : 'warning'" size="tiny" style="transform: scale(0.9)">
                    {{ deviceInfo.cursorInfo.userInfo?.email_verified ? '已验证' : '未验证' }}
                  </n-tag>
                </n-space>
              </n-space>
              <n-space :size="8" style="line-height: 1.2;">
                <span style="width: 70px">注册时间</span>
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
            <!-- Cursor GPT-4 使用统计 -->
            <n-space vertical :size="8">
              <n-space justify="space-between">
                <span>高级模型使用量</span>
                <n-space v-if="deviceInfo.cursorInfo.usage" :size="0">
                  <n-number-animation 
                    :from="0" 
                    :to="deviceInfo.cursorInfo.usage['gpt-4']?.numRequests || 0"
                    :duration="1000"
                  />
                  <span>/{{ deviceInfo.cursorInfo.usage['gpt-4']?.maxRequestUsage || 0 }}</span>
                </n-space>
                <span v-else>无法获取</span>
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
                <span>普通模型使用量</span>
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
                <span v-else>无法获取</span>
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

            <!-- 账户使用统计 -->
            <n-space vertical :size="8">
              <n-space justify="space-between">
                <span>CC账户使用量</span>
                <n-space :size="0">
                  <n-number-animation 
                    :from="0" 
                    :to="deviceInfo.userInfo?.daily_used_count || 0"
                    :duration="1000"
                  />
                  <span>/{{ i18n.dashboard.unlimited }}</span>
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
          <n-button type="primary" @click="handleMachineCodeChange">
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