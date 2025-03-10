import { invoke } from '@tauri-apps/api/core'
import type {
    ApiResponse,
    LoginResponse,
    UserInfo,
    AccountPoolInfo,
    UsageInfo,
    VersionInfo,
    PublicInfo,
    MachineInfo,
    DisclaimerResponse,
    HistoryRecord,
    HistoryAccountRecord
} from './types'

// 错误处理
function handleApiResponse<T>(response: ApiResponse<T>): T {
    if (response.status === 200) {
        // 成功时返回 data
        if (response.data) {
            return response.data
        }
        // 如果没有data，返回空对象
        return {} as T
    }
    throw new Error(response.msg || 'API request failed')
}

// API 错误类
export class ApiError extends Error {
    constructor(message: string) {
        super(message)
        this.name = 'ApiError'
    }
}

// 用户认证相关 API
export async function checkUser(email: string): Promise<ApiResponse<any>> {
    try {
        const response = await invoke<ApiResponse<any>>('check_user', { email })
        return response
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to check user')
    }
}

export async function sendCode(email: string, type: string): Promise<void> {
    try {
        const response = await invoke<ApiResponse<void>>('send_code', { email, type })
        handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to send code')
    }
}

export async function register(email: string, code: string, password: string, spread: string): Promise<LoginResponse> {
    try {
        const response = await invoke<ApiResponse<LoginResponse>>('register', { email, code, password, spread })
        if (response.status === 200 && response.data?.token) {
            return response.data as LoginResponse
        }
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to register')
    }
}

export async function login(account: string, password: string, spread: string): Promise<LoginResponse> {
    try {
        const response = await invoke<ApiResponse<LoginResponse>>('login', { account, password, spread })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to login')
    }
}

// 用户信息相关 API
export async function getUserInfo(): Promise<UserInfo> {
    try {
        const response = await invoke<ApiResponse<UserInfo>>('get_user_info')
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get user info')
    }
}

export async function getAccount(account?: string, usageCount?: string): Promise<AccountPoolInfo> {
    try {
        const response = await invoke<ApiResponse<AccountPoolInfo>>('get_account', { account, usageCount })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get account info')
    }
}

// Cursor 平台相关 API
export async function getUsage(token: string): Promise<UsageInfo> {
    try {
        const response = await invoke<ApiResponse<UsageInfo>>('get_usage', { token })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get usage info')
    }
}

// 系统信息相关 API
export async function getPublicInfo(): Promise<PublicInfo> {
    try {
        const response = await invoke<ApiResponse<PublicInfo>>('get_public_info')
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get public info')
    }
}

export async function getVersion(): Promise<VersionInfo> {
    try {
        const response = await invoke<ApiResponse<VersionInfo>>('get_version')
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get version info')
    }
}

// 账户管理相关 API
export async function activate(code: string): Promise<void> {
    try {
        const response = await invoke<ApiResponse<void>>('activate', { code })
        handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to activate')
    }
}

export async function changePassword(oldPassword: string, newPassword: string): Promise<void> {
    try {
        const response = await invoke<ApiResponse<void>>('change_password', { oldPassword, newPassword })
        handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to change password')
    }
}

// 机器码和账户切换相关 API
export async function resetMachineId(params: { forceKill?: boolean, machineId?: string } = {}): Promise<boolean> {
    try {
        return await invoke<boolean>('reset_machine_id', { 
            forceKill: params.forceKill || false,
            machineId: params.machineId
        })
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to reset machine id')
    }
}

export async function switchAccount(email: string, token: string, forceKill: boolean = false): Promise<void> {
    try {
        const result = await invoke<boolean>('switch_account', { email, token, forceKill })
        if (result !== true) {
            throw new Error('切换账户失败')
        }
    } catch (error) {
        const errorMsg = error instanceof Error ? error.message : 'Failed to switch account'
        if (errorMsg.includes('Cursor进程正在运行, 请先关闭Cursor')) {
            throw new Error('请先关闭 Cursor 或选择强制终止进程')
        }
        throw error
    }
}

export async function getMachineIds(): Promise<MachineInfo> {
    try {
        return await invoke<MachineInfo>('get_machine_ids')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get machine IDs')
    }
}

export async function checkCursorRunning(): Promise<boolean> {
    try {
        return await invoke<boolean>('check_cursor_running')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to check cursor status')
    }
}

// 管理员权限相关 API
export async function checkAdminPrivileges(): Promise<boolean> {
    try {
        return await invoke<boolean>('check_admin_privileges')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to check admin privileges')
    }
}

// Hook 相关 API
export async function checkHookStatus(): Promise<boolean> {
    try {
        return await invoke<boolean>('is_hook')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to check hook status')
    }
}

export async function applyHook(forceKill: boolean = false): Promise<void> {
    try {
        await invoke<void>('hook_main_js', { forceKill })
    } catch (error) {
        const errorMsg = error instanceof Error ? error.message : 'Failed to apply hook'
        if (errorMsg.includes('Cursor进程正在运行')) {
            throw new Error('请先关闭 Cursor 或选择强制终止进程')
        }
        throw error
    }
}

export async function restoreHook(forceKill: boolean = false): Promise<void> {
    try {
        await invoke<void>('restore_hook', { forceKill })
    } catch (error) {
        const errorMsg = error instanceof Error ? error.message : 'Failed to restore hook'
        if (errorMsg.includes('Cursor进程正在运行')) {
            throw new Error('请先关闭 Cursor 或选择强制终止进程')
        }
        throw error
    }
}

export async function resetPassword(email: string, code: string, password: string): Promise<void> {
    try {
        const response = await invoke<ApiResponse<void>>('reset_password', { 
            email, 
            code, 
            password 
        })
        handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to reset password')
    }
}

// 添加新的 API 函数来检测系统是否为 Windows
export async function checkIsWindows(): Promise<boolean> {
    try {
        return await invoke<boolean>('check_is_windows');
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to check if system is Windows');
    }
}

// 获取免责声明
export async function getDisclaimer(): Promise<DisclaimerResponse> {
    try {
        const response = await invoke<ApiResponse<DisclaimerResponse>>('get_disclaimer')
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get disclaimer')
    }
}

// 添加关闭和启动Cursor的API
export async function closeCursor(): Promise<boolean> {
  return await invoke('close_cursor')
}

export async function launchCursor(): Promise<boolean> {
  return await invoke('launch_cursor')
}

// 登出
export async function logout(): Promise<void> {
    try {
        const response = await invoke<ApiResponse<void>>('logout')
        handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to logout')
    }
}

// 历史记录相关 API
export async function saveHistoryRecord(record: HistoryRecord): Promise<void> {
    try {
        await invoke<void>('save_history_record', { record })
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save history record')
    }
}

export async function saveHistoryRecords(records: HistoryRecord[]): Promise<void> {
    try {
        await invoke<void>('save_history_records', { records })
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save history records')
    }
}

export async function getHistoryRecords(): Promise<HistoryRecord[]> {
    try {
        return await invoke<HistoryRecord[]>('get_history_records')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get history records')
    }
}

/**
 * 清除历史记录
 * 通过保存空数组实现
 */
export async function clearHistoryRecords(): Promise<void> {
    try {
        await saveHistoryRecords([])
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to clear history records')
    }
}

// 历史账户相关 API
export async function saveHistoryAccount(account: HistoryAccountRecord): Promise<void> {
    try {
        await invoke<void>('save_history_account', { account })
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save history account')
    }
}

export async function getHistoryAccounts(): Promise<HistoryAccountRecord[]> {
    try {
        return await invoke<HistoryAccountRecord[]>('get_history_accounts')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get history accounts')
    }
}

export async function removeHistoryAccount(email: string): Promise<void> {
    try {
        await invoke<void>('remove_history_account', { email })
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to remove history account')
    }
}

export async function clearHistoryAccounts(): Promise<void> {
    try {
        await invoke<void>('clear_history_accounts')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to clear history accounts')
    }
}