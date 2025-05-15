// 用于处理Tauri相关功能的辅助函数
import { type } from '@tauri-apps/api/os'

// 检查当前是否在Tauri应用环境中运行
export function isTauriApp(): boolean {
  // 检查全局window对象中是否存在__TAURI_INTERNALS__
  return typeof window !== 'undefined' && 'object' === typeof (window as any).__TAURI_INTERNALS__
}

// 安全地获取当前窗口
export async function safeGetCurrentWindow() {
  if (isTauriApp()) {
    // 动态导入Window，避免在网页环境中直接引入导致错误
    const { Window } = await import('@tauri-apps/api/window')
    return new Window((window as any).__TAURI_INTERNALS__.metadata.currentWindow.label, {
      // @ts-expect-error `skip` is not defined in the public API but it is handled by the constructor
      skip: true,
    })
  }

  // 如果不是Tauri环境，返回null或模拟的窗口对象
  return null
}

// 其他Tauri相关的安全函数可以继续添加...
