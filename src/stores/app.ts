import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getVersion, getPublicInfo } from '@/api'
import type { VersionInfo, PublicInfo } from '@/api/types'
import { darkTheme } from 'naive-ui'
import { version as packageVersion } from '../../package.json'

// 支持的语言
type SupportedLanguage = 'zh-CN' | 'en-US'

export const useAppStore = defineStore('app', () => {
  // 状态
  const theme = ref<'light' | 'dark'>('light')
  const language = ref<SupportedLanguage>('zh-CN')
  const isLoading = ref(false)
  const appVersion = ref('')
  const latestVersion = ref<VersionInfo | null>(null)
  const publicInfo = ref<PublicInfo | null>(null)

  // Getters
  const isDarkMode = computed(() => theme.value === 'dark')
  const currentTheme = computed(() => isDarkMode.value ? darkTheme : null)
  const currentLocale = computed(() => language.value)
  const hasNewVersion = computed(() => {
    if (!latestVersion.value || !appVersion.value) return false
    return latestVersion.value.version !== appVersion.value
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
      const versionInfo = await getVersion()
      latestVersion.value = versionInfo
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
  }

  return {
    // 状态
    theme,
    language,
    isLoading,
    appVersion,
    latestVersion,
    publicInfo,
    
    // Getters
    isDarkMode,
    currentTheme,
    currentLocale,
    hasNewVersion,
    
    // Actions
    toggleTheme,
    setTheme,
    setLanguage,
    checkVersion,
    fetchPublicInfo,
    setLoading,
    setAppVersion,
    initAppSettings
  }
}) 