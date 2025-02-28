import { Ref } from 'vue'
import { getVersion } from '@/api'
import { open } from '@tauri-apps/plugin-shell'
import { version as LOCAL_VERSION } from '../../../../package.json'
import type { VersionInfo } from '@/api/types'
import { Window } from '@tauri-apps/api/window'

const appWindow = new Window('main')
// 版本检查的时间间隔（毫秒）
const VERSION_CHECK_INTERVAL = 3 * 60 * 60 * 1000 // 3小时

export function useVersionCheck(
  showUpdateModal: Ref<boolean>,
  versionInfo: Ref<VersionInfo | null>
) {
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
  const handleDownload = async () => {
    if (versionInfo.value?.downloadUrl) {
      const url = versionInfo.value.downloadUrl
      await open(url)
      appWindow.close()
    }
  }

  // 处理稍后更新
  const handleLater = () => {
    showUpdateModal.value = false
    // 记录关闭时间
    localStorage.setItem('last_version_check_time', Date.now().toString())
  }

  return {
    checkUpdate,
    handleDownload,
    handleLater
  }
} 