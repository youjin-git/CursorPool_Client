import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AccountDetail, UserInfo, PublicInfo } from '@/api/types'
import { getUserInfo, getAccount, getPublicInfo } from '@/api'

export const useAccountStore = defineStore('account', () => {
  // 状态
  const currentAccount = ref<AccountDetail | null>(null)
  const previousAccount = ref<AccountDetail | null>(null)
  const userInfo = ref<UserInfo | null>(null)
  const publicInfo = ref<PublicInfo | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  // 计算属性
  const isLoggedIn = computed(() => !!currentAccount.value)
  const canRollback = computed(() => !!previousAccount.value)

  // 获取公告信息
  async function fetchPublicInfo() {
    try {
      loading.value = true
      publicInfo.value = await getPublicInfo()
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取公告失败'
      throw error.value
    } finally {
      loading.value = false
    }
  }

  // 获取用户信息
  async function fetchUserInfo() {
    const api_key = localStorage.getItem('api_key')
    if (!api_key) return

    try {
      loading.value = true
      userInfo.value = await getUserInfo(api_key)
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取用户信息失败'
      throw error.value
    } finally {
      loading.value = false
    }
  }

  // 获取账户信息
  async function fetchAccount() {
    const api_key = localStorage.getItem('api_key')
    if (!api_key) return

    try {
      loading.value = true
      const response = await getAccount(api_key)
      
      // 保存当前账户信息到缓存
      if (response) {
        localStorage.setItem('cache.cursor.userId', response.user_id)
        localStorage.setItem('cache.cursor.token', response.token)
      }
      
      // 更新账户信息
      if (currentAccount.value) {
        previousAccount.value = { ...currentAccount.value }
      }
      currentAccount.value = response
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取账户信息失败'
      throw error.value
    } finally {
      loading.value = false
    }
  }

  // 切换账户
  async function switchAccount() {
    if (!currentAccount.value) return

    try {
      loading.value = true
      // 保存之前的账户信息用于回滚
      previousAccount.value = { ...currentAccount.value }
      
      // TODO: 实现切换账户的逻辑
      
      // 更新本地存储
      if (currentAccount.value) {
        localStorage.setItem('cache.cursor.userId', currentAccount.value.user_id)
        localStorage.setItem('cache.cursor.token', currentAccount.value.token)
      }
      
      // 添加历史记录
      addHistoryRecord('switch_account', '切换账户')
    } catch (err) {
      error.value = err instanceof Error ? err.message : '切换账户失败'
      throw error.value
    } finally {
      loading.value = false
    }
  }

  // 切换机器码
  async function switchMachineCode() {
    try {
      loading.value = true
      // TODO: 实现切换机器码的逻辑
      
      // 添加历史记录
      addHistoryRecord('switch_machine_code', '切换机器码')
    } catch (err) {
      error.value = err instanceof Error ? err.message : '切换机器码失败'
      throw error.value
    } finally {
      loading.value = false
    }
  }

  // 一键切换（账户+机器码）
  async function quickSwitch() {
    try {
      loading.value = true
      await switchAccount()
      await switchMachineCode()
      
      // 添加历史记录
      addHistoryRecord('quick_switch', '一键切换')
    } catch (err) {
      error.value = err instanceof Error ? err.message : '一键切换失败'
      throw error.value
    } finally {
      loading.value = false
    }
  }

  // 添加历史记录
  function addHistoryRecord(type: string, description: string) {
    const record = {
      type,
      description,
      timestamp: new Date().toISOString(),
      success: !error.value,
      error: error.value
    }
    
    // TODO: 实现历史记录的存储逻辑
    console.log('添加历史记录:', record)
  }

  // 初始化
  async function initialize() {
    await Promise.all([
      fetchPublicInfo(),
      fetchAccount().then(() => fetchUserInfo())
    ])
  }

  return {
    // 状态
    currentAccount,
    previousAccount,
    userInfo,
    publicInfo,
    loading,
    error,
    
    // 计算属性
    isLoggedIn,
    canRollback,
    
    // 方法
    initialize,
    fetchPublicInfo,
    fetchUserInfo,
    fetchAccount,
    switchAccount,
    switchMachineCode,
    quickSwitch
  }
})
