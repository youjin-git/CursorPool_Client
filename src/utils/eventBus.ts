import { listen } from '@tauri-apps/api/event'
import { useNotificationStore } from '../stores'

/**
 * 事件监听器列表
 */
const listeners: (() => void)[] = []

/**
 * 初始化所有事件监听
 */
export async function initEventListeners() {
  // 清除之前的监听器
  listeners.forEach((unlisten) => unlisten())
  listeners.length = 0

  // 添加仪表盘刷新事件监听
  const unlistenDashboardRefresh = await listen('refresh-dashboard', () => {
    // 检查当前是否在仪表盘页面
    const currentPath = window.location.pathname

    // 只检查是否为dashboard路径
    const isDashboardPage = currentPath === '/dashboard'

    if (isDashboardPage) {
      // 触发前端刷新事件
      window.dispatchEvent(new Event('refresh_dashboard_data'))
    }
  })

  listeners.push(unlistenDashboardRefresh)

  // 添加账户使用阈值警告事件监听
  const unlistenAccountUsage = await listen('account-usage-warning', (event) => {
    const notificationStore = useNotificationStore()
    const payload = event.payload as {
      data: {
        account: string
        remaining_percentage: number
      }
    }

    // 显示系统通知
    notificationStore.notify({
      title: '账户使用量警告',
      body: `当前账户${payload.data?.account || ''}高级模型剩余使用量仅剩${payload.data?.remaining_percentage || ''}%，建议切换账户。`,
      id: 1,
    })
  })

  listeners.push(unlistenAccountUsage)
}

/**
 * 销毁所有事件监听
 */
export function destroyEventListeners() {
  listeners.forEach((unlisten) => unlisten())
  listeners.length = 0
}
