import { invoke } from '@tauri-apps/api/core'
import type {
    ApiResponse,
    LoginRequest,
    LoginResponse,
    UserInfo,
    CheckUserResponse,
    SendCodeResponse,
    AccountDetail,
    UsageInfo,
    UserInfoResponse,
    VersionInfo,
    PublicInfo,
    MachineInfo,
    ActivateResponse
} from './types'

// 错误处理
function handleApiResponse<T>(response: ApiResponse<T>): T {
    if (response.status === 'success' && response.data) {
        return response.data
    }
    throw new Error(response.message || 'API request failed')
}

// API 错误类
export class ApiError extends Error {
    constructor(message: string) {
        super(message)
        this.name = 'ApiError'
    }
}

// 用户认证相关 API
export async function checkUser(username: string): Promise<CheckUserResponse> {
    try {
        const response = await invoke<ApiResponse<CheckUserResponse>>('check_user', { username })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to check user')
    }
}

export async function sendCode(username: string): Promise<SendCodeResponse> {
    try {
        const response = await invoke<ApiResponse<SendCodeResponse>>('send_code', { username })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to send code')
    }
}

export async function login(params: LoginRequest): Promise<LoginResponse> {
    try {
        const response = await invoke<LoginResponse>('login', {
            username: params.username,
            password: params.password,
            deviceId: params.deviceId,
            smsCode: params.smsCode
        })
        return response
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to login')
    }
}

// 用户信息相关 API
export async function getUserInfo(apiKey: string): Promise<UserInfo> {
    try {
        const response = await invoke<ApiResponse<UserInfo>>('get_user_info', { apiKey })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get user info')
    }
}

export async function getAccount(apiKey: string): Promise<AccountDetail> {
    try {
        const response = await invoke<ApiResponse<AccountDetail>>('get_account', { apiKey })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get account info')
    }
}

// Cursor 平台相关 API
export async function getUserInfoCursor(userId: string, token: string): Promise<UserInfoResponse> {
    try {
        const response = await invoke<ApiResponse<UserInfoResponse>>('get_user_info_cursor', { userId, token })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get cursor user info')
    }
}

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
export async function activate(apiKey: string, code: string): Promise<ActivateResponse> {
    try {
        const response = await invoke<ApiResponse<ActivateResponse>>('activate', { apiKey, code })
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to activate')
    }
}

export async function changePassword(apiKey: string, old_password: string, new_password: string): Promise<LoginResponse> {
    try {
        const response = await invoke<ApiResponse<LoginResponse>>(
            'change_password',
            {
                apiKey,
                oldPassword: old_password,
                newPassword: new_password,
            }
        )
        return handleApiResponse(response)
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to change password')
    }
}

// 机器码和账户切换相关 API
export async function resetMachineIdOnly(): Promise<void> {
    try {
        await invoke<void>('reset_machine_id_only')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to reset machine ID')
    }
}

export async function switchAccount(email: string, token: string): Promise<void> {
    try {
        const result = await invoke<boolean>('switch_account', { email, token })
        if (result !== true) {
            throw new Error('切换账户失败')
        }
    } catch (error) {
        console.error('Switch account error:', error)
        throw new ApiError(error instanceof Error ? error.message : 'Failed to switch account')
    }
}

export async function getCurrentAccount(): Promise<string> {
    try {
        return await invoke<string>('get_current_account')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get current account')
    }
}

export async function getMachineIds(): Promise<MachineInfo> {
    try {
        return await invoke<MachineInfo>('get_machine_ids')
    } catch (error) {
        throw new ApiError(error instanceof Error ? error.message : 'Failed to get machine IDs')
    }
}