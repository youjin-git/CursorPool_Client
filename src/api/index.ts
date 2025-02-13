import axios from 'axios'
import { useMessage } from 'naive-ui'
import { LoginResponse } from './types'

// 创建axios实例
const instance = axios.create({
  baseURL: '/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器
instance.interceptors.request.use(
  (config) => {
    // 从localStorage获取token
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  }
)

// 响应拦截器
instance.interceptors.response.use(
  (response) => {
    // 处理响应数据
    if (response.data.status === 'error') {
      throw new Error(response.data.message)
    }
    return response.data
  },
  (error) => {
    // 处理错误
    const message = useMessage()
    if (error.response) {
      switch (error.response.status) {
        case 401:
          message.error('未授权，请重新登录')
          // TODO: 跳转到登录页面
          break
        case 403:
          message.error('权限不足')
          break
        case 404:
          message.error('请求的资源不存在')
          break
        case 500:
          message.error('服务器内部错误')
          break
        default:
          message.error(error.response.data?.message || '请求失败')
      }
    } else if (error.request) {
      message.error('网络错误，请检查网络连接')
    } else {
      message.error('请求配置错误')
    }
    return Promise.reject(error)
  }
)

// 封装通用请求方法
export const request = {
  get: <T>(url: string, params?: any): Promise<T> => instance.get(url, { params }),
  post: <T>(url: string, data?: any): Promise<T> => instance.post(url, data),
  put: <T>(url: string, data?: any): Promise<T> => instance.put(url, data),
  delete: <T>(url: string, params?: any): Promise<T> => instance.delete(url, { params })
}

// 修改login函数为导出
export const login = async (username: string, password: string): Promise<LoginResponse> => {
  return request.post<LoginResponse>('/user/login', { username, password })
}

// 导出axios实例
export default instance 