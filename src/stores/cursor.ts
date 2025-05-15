import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import {
  getMachineIds,
  getUsage,
  resetMachineId,
  switchAccount,
  checkHookStatus,
  applyHook,
  restoreHook,
  closeCursor,
  launchCursor,
  checkCursorRunning,
  getAccount,
  saveHistoryRecord,
  findCursorPath,
} from '@/api'
import type { UsageInfo, MachineInfo } from '@/api/types'
import type { HistoryAccount } from '@/types/history'
import { useHistoryStore } from './history'
import { open } from '@tauri-apps/plugin-dialog'
import Logger from '../utils/logger'

export const useCursorStore = defineStore('cursor', () => {
  // 状态
  const machineCode = ref('')
  const currentAccount = ref('')
  const cursorToken = ref('')
  const cursorInfo = ref<{
    userInfo: any | null
    usage: UsageInfo | null
    errorType: string | null
  }>({
    userInfo: null,
    usage: null,
    errorType: null,
  })
  const isLoading = ref(false)
  const hookStatus = ref<boolean | null>(null)

  // 新增状态
  const operationLoading = ref(false)
  const machineCodeLoading = ref(false)
  const accountSwitchLoading = ref(false)
  const quickChangeLoading = ref(false)
  const isForceKilling = ref(false)
  const needSaveCurrentAccount = ref(false)

  // 添加文件选择模态框状态
  const showSelectFileModal = ref(false)
  const fileSelectError = ref('')
  const fileSelectLoading = ref(false)
  const pendingAction = ref<{
    type: string
    params?: any
  } | null>(null)

  // Getters
  const gpt4Usage = computed(() => {
    const usage = cursorInfo.value?.usage?.['gpt-4']
    if (!usage)
      return {
        used: 0,
        total: 0,
        percentage: 0,
      }
    return {
      used: usage.numRequests || 0,
      total: usage.maxRequestUsage || 0,
      percentage: getUsagePercentage(usage.numRequests, usage.maxRequestUsage),
    }
  })

  const gpt35Usage = computed(() => {
    const usage = cursorInfo.value?.usage?.['gpt-3.5-turbo']
    if (!usage)
      return {
        used: 0,
        total: 0,
        percentage: 0,
      }
    return {
      used: usage.numRequests || 0,
      total: usage.maxRequestUsage || 0,
      percentage: getUsagePercentage(usage.numRequests, usage.maxRequestUsage),
    }
  })

  const isHooked = computed(() => hookStatus.value === true)

  // 辅助函数
  function getUsagePercentage(used: number | null | undefined, total: number | null | undefined) {
    if (!used || !total) return 0
    return Math.min(100, Math.round((used / total) * 100))
  }

  // 格式化日期函数
  function formatDate(dateStr: string) {
    const date = new Date(dateStr)
    return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
  }

  /**
   * 安全地获取Cursor使用情况
   * 失败时只记录日志，不会抛出异常
   * @param operationName 当前执行的操作名称，用于日志记录
   * @returns 获取是否成功
   */
  async function safelyFetchCursorUsage(operationName: string): Promise<boolean> {
    try {
      await fetchCursorUsage()
      return true
    } catch (error) {
      // 仅记录日志，不影响主流程
      await Logger.info(`获取Cursor使用情况失败，但不影响${operationName}流程`)
      console.error(`获取Cursor使用情况失败:`, error)
      return false
    }
  }

  // Actions
  /**
   * 获取机器码信息
   * @returns MachineInfo 机器码信息
   */
  async function fetchMachineIds(): Promise<MachineInfo> {
    try {
      isLoading.value = true
      const result = await getMachineIds()
      machineCode.value = result.machineId
      currentAccount.value = result.currentAccount
      cursorToken.value = result.cursorToken

      // 获取 Hook 状态
      await checkHook()

      return result
    } catch (error) {
      console.error('获取机器码失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 获取 Cursor 使用量
   */
  async function fetchCursorUsage() {
    try {
      if (!cursorToken.value) {
        await fetchMachineIds()
      }

      if (!cursorToken.value) {
        console.error('未找到 Cursor Token')
        // 设置错误类型为数据库错误
        cursorInfo.value = {
          userInfo: null,
          usage: null,
          errorType: 'cursor_db_error',
        }
        return
      }

      isLoading.value = true
      const usageData = await getUsage(cursorToken.value)

      // TODO: 临时处理，Cursor高级模型使用量上限为50 适配cursor最新政策
      if (usageData && usageData['gpt-4'] && usageData['gpt-4'].maxRequestUsage === 150) {
        usageData['gpt-4'].maxRequestUsage = 50
      }

      cursorInfo.value = {
        userInfo: {
          email: currentAccount.value,
          email_verified: true,
          name: currentAccount.value.split('@')[0],
          sub: '',
          updated_at: new Date().toISOString(),
          picture: null,
        },
        usage: usageData,
        errorType: null,
      }
    } catch (error) {
      console.error('获取 Cursor 使用量失败:', error)

      // 设置适当的错误类型
      const errorMsg = error instanceof Error ? error.message : String(error)
      cursorInfo.value = {
        userInfo: cursorInfo.value?.userInfo,
        usage: null,
        errorType: errorMsg,
      }

      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 重置机器码
   */
  async function resetMachine({
    forceKill = false,
    machineId,
  }: {
    forceKill?: boolean
    machineId?: string
  } = {}) {
    try {
      machineCodeLoading.value = true
      await Logger.info('开始重置机器码')

      // 检查 Cursor 是否在运行
      await ensureCursorNotRunning(forceKill)

      await resetMachineId({
        forceKill,
        machineId,
      })
      await Logger.info('机器码重置成功')

      // 添加历史记录
      await saveHistoryRecord({
        id: Date.now(),
        type_name: '机器码修改',
        detail: `修改机器码: ${machineCode.value}`,
        timestamp: new Date().toISOString(),
        operator: '用户',
      })

      await fetchMachineIds()
      await safelyFetchCursorUsage('机器码重置')

      return true
    } catch (error) {
      await Logger.error(`重置机器码失败: ${error}`)
      throw error
    } finally {
      isLoading.value = false
      machineCodeLoading.value = false
    }
  }

  /**
   * 切换账户
   */
  async function switchCursorAccount(email?: string, token?: string, forceKill: boolean = false) {
    try {
      accountSwitchLoading.value = true
      await Logger.info('开始切换账户操作')
      await Logger.info('开始切换账户操作1')

      // 检查 Cursor 是否在运行
      // await ensureCursorNotRunning(forceKill)

      // 如果未提供邮箱和token，则自动获取

      const accountInfo = await getAccount(undefined, '1')
      if (!accountInfo.account_info.account || !accountInfo.account_info.token) {
        await Logger.error('获取账户信息失败，无法进行切换')
        throw new Error('获取账户信息失败')
      }
      email = accountInfo.account_info.account
      token = accountInfo.account_info.token

      await switchAccount(email, token, forceKill)
      await Logger.info(`账户切换成功: ${email}`)

      // 添加历史记录
      await saveHistoryRecord({
        id: Date.now(),
        type_name: '账户切换',
        detail: `切换到账户: ${email} 扣除50积分`,
        timestamp: new Date().toISOString(),
        operator: '用户',
      })

      await fetchMachineIds()
      await safelyFetchCursorUsage('账户切换')

      return true
    } catch (error) {
      await Logger.error(`账户切换失败: ${error}`)
      throw error
    } finally {
      isLoading.value = false
      accountSwitchLoading.value = false
    }
  }

  /**
   * 一键更换（账户+机器码）
   */
  async function quickChange(email?: string, token?: string, forceKill: boolean = false) {
    try {
      quickChangeLoading.value = true
      await Logger.info('开始一键换号操作')

      // 检查 Cursor 是否在运行
      await ensureCursorNotRunning(forceKill)

      // 保存原始机器码，以便失败时恢复
      const originalMachineId = machineCode.value

      let machineResetSuccess = false

      // 先重置机器码
      try {
        await resetMachine({
          forceKill,
        })
        machineResetSuccess = true
      } catch (error) {
        await Logger.error('一键换号时重置机器码失败')
        throw error
      }

      // 再切换账户
      try {
        await switchCursorAccount(email, token, forceKill)
        await Logger.info('一键换号完成')
      } catch (error) {
        await Logger.error('一键换号时切换账户失败')

        // 机器码重置成功 但账户切换失败 恢复原始机器码
        if (machineResetSuccess && originalMachineId) {
          try {
            await resetMachineId({
              forceKill,
              machineId: originalMachineId,
            })
            await Logger.info('恢复原始机器码成功')
            await fetchMachineIds() // 刷新机器码信息
          } catch (restoreError) {
            await Logger.error(`恢复原始机器码失败: ${restoreError}`)
          }
        }

        throw error
      }

      await fetchMachineIds()
      await safelyFetchCursorUsage('一键换号')

      return true
      // eslint-disable-next-line no-useless-catch
    } catch (error) {
      throw error
    } finally {
      isLoading.value = false
      quickChangeLoading.value = false
    }
  }

  /**
   * 检查Hook状态
   */
  async function checkHook() {
    try {
      // 清除先前的状态
      isLoading.value = true

      const status = await checkHookStatus()

      // 更新状态
      hookStatus.value = status
      return status
    } catch (error) {
      console.error('检查Hook状态失败:', error)
      // 如果是找不到main.js文件，不更新状态
      if (error instanceof Error && error.message.includes('MAIN_JS_NOT_FOUND')) {
        console.log('检测到main.js文件路径问题')
        // 保持现有状态
        return hookStatus.value
      }
      // 其他错误情况下重置状态
      hookStatus.value = null
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 应用 Hook
   */
  async function applyHookToClient(forceKill: boolean = false) {
    try {
      await Logger.info('开始注入Hook')
      operationLoading.value = true
      isLoading.value = true

      // 执行操作
      await applyHook(forceKill)

      // 明确设置状态为 true
      hookStatus.value = true

      // 触发检查以确保状态已更新
      await checkHook()

      await Logger.info('Hook注入成功')
      return true
    } catch (error) {
      await Logger.error(`Hook注入失败: ${error}`)
      hookStatus.value = false
      throw error
    } finally {
      isLoading.value = false
      operationLoading.value = false
    }
  }

  /**
   * 恢复 Hook
   */
  async function restoreHookFromClient(forceKill: boolean = false) {
    try {
      await Logger.info('开始恢复Hook')
      operationLoading.value = true
      isLoading.value = true

      // 执行操作
      await restoreHook(forceKill)

      // 明确设置状态为 false
      hookStatus.value = false

      // 触发检查以确保状态已更新
      await checkHook()

      await Logger.info('Hook恢复成功')
      return true
    } catch (error) {
      await Logger.error(`Hook恢复失败: ${error}`)
      throw error
    } finally {
      isLoading.value = false
      operationLoading.value = false
    }
  }

  /**
   * 关闭 Cursor
   */
  async function closeCursorApp() {
    try {
      operationLoading.value = true
      return await closeCursor()
    } catch (error) {
      console.error('关闭 Cursor 失败:', error)
      throw error
    } finally {
      operationLoading.value = false
    }
  }

  /**
   * 启动 Cursor
   */
  async function launchCursorApp() {
    try {
      operationLoading.value = true
      return await launchCursor()
    } catch (error) {
      console.error('启动 Cursor 失败:', error)
      throw error
    } finally {
      operationLoading.value = false
    }
  }

  /**
   * 检查是否需要注入Hook并自动注入
   */
  async function ensureHookApplied() {
    // 检查 Hook 状态
    await checkHook()

    // 如果未注入，尝试自动注入
    if (!hookStatus.value) {
      return await applyHookToClient(false)
    }

    return true
  }

  /**
   * 刷新所有Cursor相关数据
   */
  async function refreshAllCursorData() {
    try {
      isLoading.value = true
      await fetchMachineIds()
      await safelyFetchCursorUsage('数据刷新')

      return true
    } catch (error) {
      console.error('刷新数据失败:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  /**
   * 切换到历史账户
   */
  async function switchToHistoryAccount(account: HistoryAccount) {
    const historyStore = useHistoryStore()
    historyStore.switchingAccount[account.email] = true

    try {
      // 检查Cursor是否在运行
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        // 返回需要处理的状态
        return {
          status: 'running',
          account,
        }
      }

      // 检查钩子状态
      const hookStatus = await checkHookStatus()
      if (!hookStatus) {
        const hookSuccess = await applyHookToClient(false)
        if (!hookSuccess) {
          return {
            status: 'hook_failed',
          }
        }
      }

      // 切换账户 - 后端会自动保存历史记录
      await resetMachineId({
        machineId: account.machineCode,
      })
      await switchAccount(account.email, account.token, false)

      await saveHistoryRecord({
        id: Date.now(),
        type_name: '历史账户切换',
        detail: `切换到历史账户: ${account.email}`,
        timestamp: new Date().toISOString(),
        operator: '用户',
      })

      await fetchMachineIds()
      await safelyFetchCursorUsage('历史账户切换')

      return {
        status: 'success',
      }
    } catch (error) {
      console.error('切换到历史账户失败:', error)
      throw error
    } finally {
      historyStore.switchingAccount[account.email] = false
    }
  }

  /**
   * 强制关闭并切换账户
   */
  async function forceCloseAndSwitch(account: HistoryAccount) {
    const historyStore = useHistoryStore()
    historyStore.switchingAccount[account.email] = true
    isForceKilling.value = true

    try {
      // 关闭 Cursor
      await closeCursorApp()
      await new Promise((resolve) => setTimeout(resolve, 1000))

      // 检查并应用钩子
      if (!(await checkHookStatus())) {
        const hookSuccess = await applyHookToClient(true)
        if (!hookSuccess) {
          return {
            status: 'hook_failed',
          }
        }
      }

      // 账户切换 - 后端会自动保存历史记录
      await resetMachineId({
        machineId: account.machineCode,
      })
      await switchAccount(account.email, account.token, true)

      await saveHistoryRecord({
        id: Date.now(),
        type_name: '历史账户切换',
        detail: `切换到历史账户: ${account.email}`,
        timestamp: new Date().toISOString(),
        operator: '用户',
      })

      await fetchMachineIds()
      await safelyFetchCursorUsage('强制切换账户')

      // 启动 Cursor
      await launchCursorApp()

      return { status: 'success' }
    } catch (error) {
      console.error('强制切换账户失败:', error)
      throw error
    } finally {
      needSaveCurrentAccount.value = false
      isForceKilling.value = false
      historyStore.switchingAccount[account.email] = false
    }
  }

  /**
   * 处理文件选择
   */
  async function handleSelectCursorPath() {
    // 不在这里调用useMessage，而是通过外部传入或通过事件处理
    if (fileSelectLoading.value) return

    fileSelectLoading.value = true
    fileSelectError.value = ''

    try {
      // 调用文件选择对话框
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Cursor程序',
            extensions: ['exe'],
          },
          {
            name: 'JavaScript文件',
            extensions: ['js'],
          },
          {
            name: '所有文件',
            extensions: ['*'],
          },
        ],
      })

      // 检查用户是否取消了选择
      if (!selected) {
        fileSelectLoading.value = false
        return
      }

      console.log('用户选择的文件路径:', selected)

      // 调用API处理选择的文件路径
      const result = await findCursorPath(selected as string)

      console.log('findCursorPath结果:', result)

      if (result) {
        showSelectFileModal.value = false

        // 如果有待处理的操作，执行它
        if (pendingAction.value) {
          // 保存然后清空待处理操作
          const currentAction = {
            ...pendingAction.value,
          }
          pendingAction.value = null

          try {
            // 根据待处理操作类型执行相应的方法
            switch (currentAction.type) {
              case 'applyHook':
                await applyHookToClient(currentAction.params?.forceKill || false)
                break
              case 'restoreHook':
                await restoreHookFromClient(currentAction.params?.forceKill || false)
                break
              // 可以添加其他操作类型的处理...
            }

            // 强制重新获取Hook状态以刷新UI
            await checkHook()
          } catch (actionError) {
            console.error('执行操作失败:', actionError)
            fileSelectError.value =
              '执行操作失败: ' +
              (actionError instanceof Error ? actionError.message : String(actionError))
          }
        }

        // 操作完成后，设置加载状态为false
        fileSelectLoading.value = false
      } else {
        throw new Error('无法验证所选择的文件路径')
      }
    } catch (error) {
      console.error('文件选择处理错误:', error)
      fileSelectError.value = error instanceof Error ? error.message : String(error)
      fileSelectLoading.value = false
    }
  }

  /**
   * 设置待执行的操作
   */
  function setPendingAction(type: string, params?: any) {
    pendingAction.value = {
      type,
      params,
    }
    showSelectFileModal.value = true
  }

  /**
   * 检查Cursor是否正在运行，如果正在运行且不允许强制关闭则抛出错误
   */
  async function ensureCursorNotRunning(forceKill: boolean) {
    if (!forceKill && (await checkCursorRunning())) {
      throw new Error('Cursor进程正在运行, 请先关闭Cursor')
    }
  }

  return {
    // 状态
    machineCode,
    currentAccount,
    cursorToken,
    cursorInfo,
    isLoading,
    hookStatus,
    operationLoading,
    machineCodeLoading,
    accountSwitchLoading,
    quickChangeLoading,
    isForceKilling,
    needSaveCurrentAccount,

    // 添加文件选择模态框状态
    showSelectFileModal,
    fileSelectError,
    fileSelectLoading,
    pendingAction,

    // Getters
    gpt4Usage,
    gpt35Usage,
    isHooked,
    formatDate,

    // Actions
    fetchMachineIds,
    fetchCursorUsage,
    resetMachine,
    switchCursorAccount,
    quickChange,
    checkHook,
    applyHookToClient,
    restoreHookFromClient,
    closeCursorApp,
    launchCursorApp,
    ensureHookApplied,
    refreshAllCursorData,
    switchToHistoryAccount,
    forceCloseAndSwitch,

    // 添加文件选择相关方法
    handleSelectCursorPath,
    setPendingAction,
  }
})
