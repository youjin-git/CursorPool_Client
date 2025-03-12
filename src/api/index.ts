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

// 使用键值存储实现历史记录功能

/**
 * 保存历史记录
 * @param record 历史记录
 */
export async function saveHistoryRecord(record: HistoryRecord): Promise<void> {
    try {
        // 先获取现有记录
        let records = await getHistoryRecords();
        
        // 添加新记录
        records.push(record);
        
        // 保存回数据库
        await setUserData('user.history', JSON.stringify(records));
    } catch (error) {
        console.error('保存历史记录失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save history record');
    }
}

/**
 * 批量保存历史记录
 * @param records 历史记录数组
 */
export async function saveHistoryRecords(records: HistoryRecord[]): Promise<void> {
    try {
        // 先获取现有记录
        let existingRecords = await getHistoryRecords();
        
        // 合并记录
        existingRecords = [...existingRecords, ...records];
        
        // 保存回数据库
        await setUserData('user.history', JSON.stringify(existingRecords));
    } catch (error) {
        console.error('批量保存历史记录失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save history records');
    }
}

/**
 * 获取所有历史记录
 * @returns 历史记录数组
 */
export async function getHistoryRecords(): Promise<HistoryRecord[]> {
    try {
        const data = await getUserData('user.history');
        if (!data) {
            return [];
        }
        
        try {
            return JSON.parse(data) as HistoryRecord[];
        } catch (e) {
            console.error('历史记录解析失败:', e);
            return [];
        }
    } catch (error) {
        console.error('获取历史记录失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get history records');
    }
}

/**
 * 清除所有历史记录
 */
export async function clearHistoryRecords(): Promise<void> {
    try {
        await delUserData('user.history');
    } catch (error) {
        console.error('清除历史记录失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to clear history records');
    }
}

/**
 * 保存历史账户
 * @param account 历史账户记录
 */
export async function saveHistoryAccount(account: HistoryAccountRecord): Promise<void> {
    try {
        // 先获取现有账户
        let accounts = await getHistoryAccounts();
        
        // 检查是否存在相同email的账户，存在则更新，不存在则添加
        const index = accounts.findIndex(a => a.email === account.email);
        if (index !== -1) {
            accounts[index] = account;
        } else {
            accounts.push(account);
        }
        
        // 保存回数据库
        await setUserData('user.history.accounts', JSON.stringify(accounts));
    } catch (error) {
        console.error('保存历史账户失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save history account');
    }
}

/**
 * 获取所有历史账户
 * @returns 历史账户数组
 */
export async function getHistoryAccounts(): Promise<HistoryAccountRecord[]> {
    try {
        const data = await getUserData('user.history.accounts');
        if (!data) {
            return [];
        }
        
        try {
            return JSON.parse(data) as HistoryAccountRecord[];
        } catch (e) {
            console.error('历史账户解析失败:', e);
            return [];
        }
    } catch (error) {
        console.error('获取历史账户失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get history accounts');
    }
}

/**
 * 删除历史账户
 * @param email 账户邮箱
 */
export async function removeHistoryAccount(email: string): Promise<void> {
    try {
        // 先获取现有账户
        let accounts = await getHistoryAccounts();
        
        // 过滤掉要删除的账户
        accounts = accounts.filter(a => a.email !== email);
        
        // 保存回数据库
        await setUserData('user.history.accounts', JSON.stringify(accounts));
    } catch (error) {
        console.error('删除历史账户失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to remove history account');
    }
}

/**
 * 清除所有历史账户
 */
export async function clearHistoryAccounts(): Promise<void> {
    try {
        await delUserData('user.history.accounts');
    } catch (error) {
        console.error('清除历史账户失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to clear history accounts');
    }
}

/**
 * 保存用户API Token
 * @param token API Token
 */
export async function saveUserApiToken(token: string): Promise<void> {
    try {
        await setUserData('user.info.token', token);
    } catch (error) {
        console.error('保存API Token失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to save API token');
    }
}

/**
 * 获取用户API Token
 * @returns API Token，如果不存在则返回null
 */
export async function getUserApiToken(): Promise<string | null> {
    try {
        return await getUserData('user.info.token');
    } catch (error) {
        console.error('获取API Token失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get API token');
    }
}

/**
 * 清除用户API Token
 */
export async function clearUserApiToken(): Promise<void> {
    try {
        await delUserData('user.info.token');
    } catch (error) {
        console.error('清除API Token失败:', error);
        throw new ApiError(error instanceof Error ? error.message : 'Failed to clear API token');
    }
}

// 添加通用的键值存储 API 方法

/**
 * 设置用户数据
 * @param key 键名
 * @param value 值
 */
export async function setUserData(key: string, value: string): Promise<void> {
    try {
        await invoke<ApiResponse<any>>('set_user_data', { key, value });
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to set user data');
    }
}

/**
 * 获取用户数据
 * @param key 键名
 * @returns 获取的值，如果不存在则返回 null
 */
export async function getUserData(key: string): Promise<string | null> {
    try {
        const response = await invoke<ApiResponse<{ value: string | null }>>('get_user_data', { key });
        const result = handleApiResponse(response);
        return result.value;
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get user data');
    }
}

/**
 * 删除用户数据
 * @param key 键名
 */
export async function delUserData(key: string): Promise<void> {
    try {
        await invoke<ApiResponse<any>>('del_user_data', { key });
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to delete user data');
    }
}

// 使用通用 API 实现的特定功能

/**
 * 检查用户是否已接受免责声明
 * @returns 是否已接受
 */
export async function checkDisclaimerAccepted(): Promise<boolean> {
    try {
        const value = await getUserData('user.disclaimer.accepted');
        return value === 'true';
    } catch (error) {
        console.error('检查免责声明失败:', error);
        return false;
    }
}

/**
 * 设置用户已接受免责声明
 */
export async function setDisclaimerAccepted(): Promise<void> {
    try {
        await setUserData('user.disclaimer.accepted', 'true');
    } catch (error) {
        console.error('设置免责声明状态失败:', error);
        throw error;
    }
}

/**
 * 清除用户的免责声明接受状态
 */
export async function clearDisclaimerAccepted(): Promise<void> {
    try {
        await delUserData('user.disclaimer.accepted');
    } catch (error) {
        console.error('清除免责声明状态失败:', error);
        throw error;
    }
}