import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getVersion, getPublicInfo, checkDisclaimerAccepted, setDisclaimerAccepted, getUserData, setUserData } from '@/api'
import type { VersionInfo, PublicInfo } from '@/api/types'
import { darkTheme } from 'naive-ui'
import { version as packageVersion } from '../../package.json'
import { open } from '@tauri-apps/plugin-shell'

// 支持的语言
type SupportedLanguage = 'zh-CN' | 'en-US'

// 版本检查的时间间隔（毫秒）
const VERSION_CHECK_INTERVAL = 3 * 60 * 60 * 1000 // 3小时

// 免责声明内容
const DISCLAIMER_CONTENT = `Cursor Pool 使用须知

欢迎使用Cursor Pool! 我们的产品可以让您的本地Cursor AI编辑器实现无缝刷新账号。
本软件执行过程中会对Cursor客户端的主要执行文件进行hook,以达到修改Cursor账号的目的。
本软件会修改Cursor客户端在本地生成的文件进行修改或修改权限,以达到修改Cursor机器码的目的。

同意使用Cursor Pool即表示您授权我们hook Cursor客户端和修改机器码的权限
Cursor Pool 客户端免费开源, 如果你使用第三方客户端禁止使用Cursor Pool的:
1. 名称
2. 图标
同时 设置页面必须保留源项目开源作者声明
如果您同意上述操作和授权，请点击"同意并继续"。`

export const useAppStore = defineStore('app', () => {
  // 状态
  const theme = ref<'light' | 'dark'>('light')
  const language = ref<SupportedLanguage>('zh-CN')
  const isLoading = ref(false)
  const appVersion = ref('')
  const latestVersion = ref<VersionInfo | null>(null)
  const publicInfo = ref<PublicInfo | null>(null)
  
  // 新增状态
  const showUpdateModal = ref(false)
  const showDisclaimerModal = ref(false)
  const disclaimerContent = ref(DISCLAIMER_CONTENT)
  const disclaimerCountdown = ref(3)
  const canConfirmDisclaimer = ref(false)
  const disclaimerLoading = ref(false)
  
  // 引导状态
  const tourAccepted = ref<string | null>(null)
  const tourLoading = ref(false)

  // Getters
  const isDarkMode = computed(() => theme.value === 'dark')
  const currentTheme = computed(() => isDarkMode.value ? darkTheme : null)
  const currentLocale = computed(() => language.value)
  const hasNewVersion = computed(() => {
    if (!latestVersion.value || !appVersion.value) return false
    return compareVersions(appVersion.value, latestVersion.value.version) < 0
  })
  
  // 引导状态的计算属性
  const shouldShowTour = computed(() => {
    return tourAccepted.value !== 'true'
  })

  // Actions
  /**
   * 切换主题
   */
  function toggleTheme() {
    theme.value = theme.value === 'light' ? 'dark' : 'light'
    localStorage.setItem('theme', theme.value)
  }

  /**
   * 设置主题
   */
  function setTheme(newTheme: 'light' | 'dark') {
    theme.value = newTheme
    localStorage.setItem('theme', theme.value)
  }

  /**
   * 设置语言
   */
  function setLanguage(newLanguage: SupportedLanguage) {
    language.value = newLanguage
    localStorage.setItem('language', language.value)
  }

  /**
   * 检查版本
   */
  async function checkVersion() {
    try {
      isLoading.value = true
      
      // 检查上次更新提示的时间
      const lastCheckTime = localStorage.getItem('last_version_check_time')
      const now = Date.now()
      
      // 如果上次检查时间存在且距离现在小于设定的间隔，则跳过检查
      if (lastCheckTime) {
        const timeDiff = now - parseInt(lastCheckTime)
        if (timeDiff < VERSION_CHECK_INTERVAL) {
          return null
        }
      }
      
      const versionInfo = await getVersion()
      latestVersion.value = versionInfo
      
      // 如果有新版本，显示更新提示
      if (hasNewVersion.value) {
        showUpdateModal.value = true
        
        // 只有在非强制更新时才更新检查时间
        if (!versionInfo.forceUpdate) {
          localStorage.setItem('last_version_check_time', now.toString())
        }
      }
      
      return versionInfo
    } catch (error) {
      console.error('检查版本失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 获取公共信息
   */
  async function fetchPublicInfo() {
    try {
      isLoading.value = true
      const info = await getPublicInfo()
      publicInfo.value = info
      return info
    } catch (error) {
      console.error('获取公共信息失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 设置加载状态
   */
  function setLoading(loading: boolean) {
    isLoading.value = loading
  }

  /**
   * 设置应用版本
   */
  function setAppVersion(version: string) {
    appVersion.value = version
  }
  
  /**
   * 获取引导状态
   */
  async function fetchTourStatus() {
    try {
      tourLoading.value = true
      const response = await getUserData('user.tour.accepted')
      
      console.log('获取引导状态返回:', response)
      
      // 处理不同类型的响应
      if (response === null) {
        // 如果状态为null，设置为false
        console.log('引导状态为null，设置为false')
        await setTourStatus('false')
        tourAccepted.value = 'false'
      } else if (typeof response === 'object' && response !== null) {
        // 尝试处理对象格式的响应
        try {
          // 使用类型断言避免TypeScript错误
          const responseObj = response as any;
          
          if (responseObj.data && responseObj.data.value === null) {
            console.log('引导状态响应对象中data.value为null，设置为false')
            await setTourStatus('false')
            tourAccepted.value = 'false'
          }
        } catch (err) {
          console.error('处理引导状态响应对象失败:', err)
          tourAccepted.value = 'false'
        }
      } else if (typeof response === 'string') {
        tourAccepted.value = response
      } else {
        // 处理其他情况
        console.log('引导状态未知类型，默认设置为false')
        await setTourStatus('false')
        tourAccepted.value = 'false'
      }
      
      return tourAccepted.value
    } catch (error) {
      console.error('获取引导状态失败:', error)
      return null
    } finally {
      tourLoading.value = false
    }
  }
  
  /**
   * 设置引导状态
   */
  async function setTourStatus(status: string) {
    try {
      tourLoading.value = true
      await setUserData('user.tour.accepted', status)
      tourAccepted.value = status
      return true
    } catch (error) {
      console.error('设置引导状态失败:', error)
      return false
    } finally {
      tourLoading.value = false
    }
  }
  
  /**
   * 完成引导
   */
  async function completeTour() {
    return await setTourStatus('true')
  }

  /**
   * 初始化应用设置
   */
  function initAppSettings() {
    // 从本地存储加载主题设置
    const savedTheme = localStorage.getItem('theme')
    if (savedTheme === 'light' || savedTheme === 'dark') {
      theme.value = savedTheme
    } else {
      // 默认使用系统主题
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      theme.value = prefersDark ? 'dark' : 'light'
    }

    // 从本地存储加载语言设置
    const savedLanguage = localStorage.getItem('language')
    if (savedLanguage === 'zh-CN' || savedLanguage === 'en-US') {
      language.value = savedLanguage as SupportedLanguage
    }

    // 从 package.json 加载应用版本
    try {
      // 直接从 package.json 读取版本号
      const version = packageVersion
      setAppVersion(version)
    } catch (error) {
      console.error('加载应用版本失败:', error)
    }
    
    // 初始化引导状态
    fetchTourStatus().catch(error => {
      console.error('初始化引导状态失败:', error)
    })
  }
  
  /**
   * 版本比较函数
   */
  function compareVersions(v1: string, v2: string) {
    const parts1 = v1.split('.').map(Number)
    const parts2 = v2.split('.').map(Number)
    
    for (let i = 0; i < Math.max(parts1.length, parts2.length); i++) {
      const a = parts1[i] || 0
      const b = parts2[i] || 0
      if (a > b) return 1
      if (a < b) return -1
    }
    return 0
  }
  
  /**
   * 处理下载更新
   */
  async function handleDownload() {
    if (latestVersion.value?.downloadUrl) {
      try {
        const url = 'https://downloader-cursor.deno.dev/'
        await open(url)
        return true
      } catch (error) {
        console.error('打开下载链接失败:', error)
        throw error
      }
    }
    return false
  }
  
  /**
   * 处理稍后更新
   */
  function handleLater() {
    showUpdateModal.value = false
    // 记录关闭时间
    localStorage.setItem('last_version_check_time', Date.now().toString())
  }
  
  /**
   * 获取免责声明（检查数据库中是否已接受）
   */
  async function fetchDisclaimer() {
    try {
      disclaimerLoading.value = true
      
      // 在应用启动时尝试删除本地存储的免责声明状态
      localStorage.removeItem('disclaimer_accepted')
      
      // 启动倒计时
      const timer = setInterval(() => {
        disclaimerCountdown.value--
        if (disclaimerCountdown.value <= 0) {
          canConfirmDisclaimer.value = true
          clearInterval(timer)
        }
      }, 1000)
      
      // 从数据库检查是否已接受免责声明
      const accepted = await checkDisclaimerAccepted()
      if (!accepted) {
        showDisclaimerModal.value = true
      }
      
      return disclaimerContent.value
    } catch (error) {
      console.error('获取免责声明失败:', error)
      throw error
    } finally {
      disclaimerLoading.value = false
    }
  }
  
  /**
   * 确认免责声明
   */
  async function confirmDisclaimer() {
    try {
      // 将接受状态保存到数据库
      await setDisclaimerAccepted()
      showDisclaimerModal.value = false
      
      // 确认免责声明后检查引导状态
      await new Promise(resolve => setTimeout(resolve, 300))
      
      // 获取最新的引导状态
      await fetchTourStatus()
      
      return true
    } catch (error) {
      console.error('保存免责声明状态失败:', error)
      return false
    }
  }

  return {
    // 状态
    theme,
    language,
    isLoading,
    appVersion,
    latestVersion,
    publicInfo,
    showUpdateModal,
    showDisclaimerModal,
    disclaimerContent,
    disclaimerCountdown,
    canConfirmDisclaimer,
    disclaimerLoading,
    tourAccepted,
    tourLoading,
    
    // Getters
    isDarkMode,
    currentTheme,
    currentLocale,
    hasNewVersion,
    shouldShowTour,
    
    // Actions
    toggleTheme,
    setTheme,
    setLanguage,
    checkVersion,
    fetchPublicInfo,
    setLoading,
    setAppVersion,
    initAppSettings,
    compareVersions,
    handleDownload,
    handleLater,
    fetchDisclaimer,
    confirmDisclaimer,
    fetchTourStatus,
    setTourStatus,
    completeTour
  }
}) 