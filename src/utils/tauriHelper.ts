// 用于处理Tauri相关功能的辅助函数

/**
 * 检查当前是否在Tauri应用环境中运行
 * @returns {boolean} 是否在Tauri环境中
 */
export function isTauriApp(): boolean {
  try {
    // 检查全局window对象中是否存在__TAURI_INTERNALS__
    return (
      typeof window !== 'undefined' &&
      'object' === typeof (window as any).__TAURI_INTERNALS__ &&
      (window as any).__TAURI_INTERNALS__ !== null
    )
  } catch (e) {
    // 如果访问__TAURI_INTERNALS__出错，则肯定不是Tauri环境
    console.warn('检测Tauri环境时出错:', e)
    return false
  }
}

// 窗口接口模拟，用于Web环境
interface WindowLike {
  label: string
  maximize: () => Promise<void>
  minimize: () => Promise<void>
  close: () => Promise<void>
  hide: () => Promise<void>
  show: () => Promise<void>
  setFocus: () => Promise<void>
  // 可以根据需要添加更多方法
}

/**
 * 创建Web环境下的窗口模拟对象
 * @returns {WindowLike} 模拟的窗口对象
 */
function createWebWindowMock(): WindowLike {
  return {
    label: 'web-window',
    maximize: async () => console.log('Web环境: 窗口最大化操作'),
    minimize: async () => console.log('Web环境: 窗口最小化操作'),
    close: async () => console.log('Web环境: 窗口关闭操作'),
    hide: async () => console.log('Web环境: 窗口隐藏操作'),
    show: async () => console.log('Web环境: 窗口显示操作'),
    setFocus: async () => console.log('Web环境: 窗口设置焦点操作'),
  }
}

/**
 * 安全地获取当前窗口，在Web环境中返回模拟对象
 * @returns {Promise<WindowLike>} 窗口对象或模拟对象
 */
export async function safeGetCurrentWindow(): Promise<WindowLike> {
  try {
    if (isTauriApp()) {
      try {
        // 动态导入Window，避免在网页环境中直接引入导致错误
        const { Window } = await import('@tauri-apps/api/window')

        // 安全地访问currentWindow的label
        const tauriInternals = (window as any).__TAURI_INTERNALS__
        if (!tauriInternals?.metadata?.currentWindow?.label) {
          console.warn('Tauri内部对象结构异常，使用默认label')
          return new Window('main', {
            // @ts-expect-error `skip` 在公共API中未定义，但构造函数会处理它
            skip: true,
          })
        }

        return new Window(tauriInternals.metadata.currentWindow.label, {
          // @ts-expect-error `skip` 在公共API中未定义，但构造函数会处理它
          skip: true,
        })
      } catch (e) {
        console.error('获取Tauri窗口失败:', e)
        // 即使在Tauri环境中出错，也返回模拟对象而不是null，保持返回类型一致
        return createWebWindowMock()
      }
    }

    // 如果不是Tauri环境，返回模拟的窗口对象
    return createWebWindowMock()
  } catch (e) {
    console.error('safeGetCurrentWindow出现未预期的错误:', e)
    return createWebWindowMock()
  }
}

/**
 * 安全地执行Tauri特定的功能
 * @param {Function} tauriFunc 要在Tauri环境中执行的函数
 * @param {Function} webFallback 在网页环境中的替代函数
 * @returns {Promise<any>} 执行结果
 */
export async function safeTauriCall<T>(
  tauriFunc: () => Promise<T>,
  webFallback: () => Promise<T>,
): Promise<T> {
  try {
    if (isTauriApp()) {
      return await tauriFunc()
    } else {
      return await webFallback()
    }
  } catch (e) {
    console.error('执行Tauri调用失败:', e)
    return await webFallback()
  }
}
