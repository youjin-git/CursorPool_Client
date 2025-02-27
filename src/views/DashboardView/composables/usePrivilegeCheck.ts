import { Ref } from 'vue'
import { useMessage } from 'naive-ui'
import { checkAdminPrivileges, checkIsWindows } from '@/api'
import { Window } from '@tauri-apps/api/window'

export function usePrivilegeCheck(showAdminPrivilegeModal: Ref<boolean>) {
  const message = useMessage()

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

  return {
    checkPrivileges,
    handleExit
  }
} 