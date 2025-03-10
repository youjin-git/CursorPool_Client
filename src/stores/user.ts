import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { 
  getUserInfo, 
  login as apiLogin, 
  register as apiRegister, 
  logout as apiLogout,
  activate as apiActivate,
  changePassword as apiChangePassword,
  resetPassword as apiResetPassword
} from '@/api'
import type { UserInfo } from '@/api/types'
import { useRouter } from 'vue-router'

export const useUserStore = defineStore('user', () => {
  // 状态
  const isLoggedIn = ref(false)
  const isCheckingLogin = ref(true)
  const userInfo = ref<UserInfo | null>(null)
  const loginError = ref('')

  // 路由
  const router = useRouter()

  // Getters
  const isAdmin = computed(() => userInfo.value?.level === 5)
  const username = computed(() => userInfo.value?.username || '')
  const expiryDate = computed(() => userInfo.value?.expireTime || '')
  const memberLevel = computed(() => userInfo.value?.level || 1)

  // Actions
  /**
   * 检查用户登录状态
   */
  async function checkLoginStatus() {
    try {
      isCheckingLogin.value = true
      const info = await getUserInfo()
      userInfo.value = info
      isLoggedIn.value = true
      loginError.value = ''
    } catch (error) {
      console.error('Failed to check login status:', error)
      userInfo.value = null
      isLoggedIn.value = false
    } finally {
      isCheckingLogin.value = false
    }
  }

  /**
   * 用户登录
   */
  async function login(account: string, password: string, spread: string = 'web') {
    try {
      const response = await apiLogin(account, password, spread)
      if (response && response.token) {
        await checkLoginStatus()
        return true
      }
      return false
    } catch (error) {
      loginError.value = error instanceof Error ? error.message : '登录失败'
      throw error
    }
  }

  /**
   * 用户注册
   */
  async function register(email: string, code: string, password: string, spread: string = 'web') {
    try {
      const response = await apiRegister(email, code, password, spread)
      if (response && response.token) {
        await checkLoginStatus()
        return true
      }
      return false
    } catch (error) {
      loginError.value = error instanceof Error ? error.message : '注册失败'
      throw error
    }
  }

  /**
   * 用户登出
   */
  async function logout() {
    try {
      // 先更新状态，再调用API
      userInfo.value = null
      isLoggedIn.value = false
      loginError.value = ''
      
      // 调用登出API
      await apiLogout()
      
      // 触发一个全局事件，通知应用用户已登出
      window.dispatchEvent(new CustomEvent('user-logout'))
      
      return true
    } catch (error) {
      console.error('Logout failed:', error)
      throw error
    }
  }

  /**
   * 激活码兑换
   */
  async function activateCode(code: string) {
    try {
      await apiActivate(code)
      // 激活成功后刷新用户信息
      await checkLoginStatus()
      return true
    } catch (error) {
      console.error('Activation failed:', error)
      throw error
    }
  }

  /**
   * 修改密码
   */
  async function changePassword(oldPassword: string, newPassword: string) {
    try {
      await apiChangePassword(oldPassword, newPassword)
      // 修改密码成功后登出
      await logout()
      return true
    } catch (error) {
      console.error('Change password failed:', error)
      throw error
    }
  }

  /**
   * 重置密码
   */
  async function resetPassword(email: string, code: string, password: string) {
    try {
      await apiResetPassword(email, code, password)
      return true
    } catch (error) {
      console.error('Reset password failed:', error)
      throw error
    }
  }

  return {
    // 状态
    isLoggedIn,
    isCheckingLogin,
    userInfo,
    loginError,
    
    // Getters
    isAdmin,
    username,
    expiryDate,
    memberLevel,
    
    // Actions
    checkLoginStatus,
    login,
    register,
    logout,
    activateCode,
    changePassword,
    resetPassword
  }
}) 