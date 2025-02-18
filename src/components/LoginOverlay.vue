<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { 
  NCard, 
  NForm, 
  NFormItem, 
  NInput, 
  NButton, 
  useMessage,
  NSpace,
  NAutoComplete
} from 'naive-ui'
import { checkUser, sendCode, login } from '../api'
import type { LoginRequest } from '../api/types'

const message = useMessage()
const loading = ref(false)
const showVerifyCode = ref(false)
const countDown = ref(0)

const formData = ref({
  username: '',
  password: '',
  sms_code: '',
})

// 邮箱验证正则
const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/

// 邮箱提供商分组
const emailProviders = [
  {
    label: 'Google',
    domain: 'gmail.com'
  },
  {
    label: '腾讯',
    domain: ['qq.com', 'foxmail.com']
  },
  {
    label: 'Microsoft',
    domain: ['outlook.com', 'hotmail.com']
  },
  {
    label: '网易',
    domain: '163.com'
  }
]

// 邮箱自动完成选项
const emailOptions = computed(() => {
  const inputValue = formData.value.username
  const username = inputValue.split('@')[0]
  
  if (!username) return []
  
  return emailProviders.map(provider => ({
    type: 'group',
    label: provider.label,
    key: provider.label,
    children: Array.isArray(provider.domain)
      ? provider.domain.map(domain => ({
          label: `${username}@${domain}`,
          value: `${username}@${domain}`
        }))
      : [{
          label: `${username}@${provider.domain}`,
          value: `${username}@${provider.domain}`
        }]
  }))
})

// 验证邮箱格式
function isValidEmail(email: string): boolean {
  if (!emailRegex.test(email)) return false
  const domain = email.split('@')[1]
  return emailProviders.some(provider => 
    Array.isArray(provider.domain)
      ? provider.domain.includes(domain)
      : provider.domain === domain
  )
}

// 检查用户是否存在的防抖定时器
let checkUserTimer: number | null = null

// 监听用户名变化
watch(() => formData.value.username, async (newValue) => {
  if (!newValue || !isValidEmail(newValue)) {
    showVerifyCode.value = false
    return
  }
  
  // 防抖处理
  if (checkUserTimer) clearTimeout(checkUserTimer)
  checkUserTimer = setTimeout(async () => {
    try {
      const result = await checkUser(newValue)
      showVerifyCode.value = !result.exists || result.need_code
    } catch (error) {
      console.error('Check user failed:', error)
    }
  }, 500)
})

// 发送验证码
async function handleSendCode() {
  if (!formData.value.username || !isValidEmail(formData.value.username)) {
    message.error('请输入有效的邮箱地址')
    return
  }
  
  try {
    loading.value = true
    const result = await sendCode(formData.value.username)
    message.success('验证码已发送')
    
    // 开始倒计时
    countDown.value = result.expire_in
    const timer = setInterval(() => {
      countDown.value--
      if (countDown.value <= 0) {
        clearInterval(timer)
      }
    }, 1000)
  } catch (error) {
    message.error('发送验证码失败')
  } finally {
    loading.value = false
  }
}

// 登录
async function handleLogin() {
  if (!formData.value.username || !isValidEmail(formData.value.username)) {
    message.error('请输入有效的邮箱地址')
    return
  }

  if (!formData.value.password) {
    message.error('请输入密码')
    return
  }

  try {
    loading.value = true
    const deviceId = 'device-' + Math.random().toString(36).substr(2, 9)
    
    const loginParams: LoginRequest = {
      username: formData.value.username,
      password: formData.value.password,
      deviceId: deviceId,
      smsCode: formData.value.sms_code || undefined
    }

    const result = await login(loginParams)
    if (result.api_key) {
      localStorage.setItem('api_key', result.api_key)
      message.success('登录成功')
      // 触发登录成功事件
      emit('login-success')
    } else {
      message.error('登录失败：未获取到API密钥')
    }
  } catch (error) {
    message.error('登录失败：' + (error instanceof Error ? error.message : '未知错误'))
  } finally {
    loading.value = false
  }
}

const emit = defineEmits(['login-success'])
</script>

<template>
  <div class="login-overlay">
    <n-card title="登录" class="login-card">
      <n-form>
        <n-form-item label="邮箱">
          <n-auto-complete
            v-model:value="formData.username"
            :options="emailOptions"
            placeholder="请输入邮箱"
            :disabled="loading"
          />
        </n-form-item>
        
        <n-form-item label="密码">
          <n-input 
            v-model:value="formData.password"
            type="password"
            placeholder="请输入密码"
            :disabled="loading"
          />
        </n-form-item>

        <n-form-item v-if="showVerifyCode" label="验证码">
          <n-space>
            <n-input 
              v-model:value="formData.sms_code"
              placeholder="请输入验证码"
              :disabled="loading"
            />
            <n-button 
              :disabled="loading || countDown > 0 || !isValidEmail(formData.username)"
              @click="handleSendCode"
            >
              {{ countDown > 0 ? `${countDown}s` : '发送验证码' }}
            </n-button>
          </n-space>
        </n-form-item>

        <n-form-item>
          <n-button 
            type="primary" 
            block 
            @click="handleLogin"
            :loading="loading"
            :disabled="!isValidEmail(formData.username)"
          >
            登录
          </n-button>
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<style scoped>
.login-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  z-index: 1000;
  user-select: none;
}

.login-card {
  width: 400px;
  max-width: 90%;
}

:deep(.n-card) {
  background: var(--n-color);
  color: var(--n-text-color);
}

:deep(.n-card-header) {
  text-align: center;
  font-size: 1.5em;
}

:deep(.n-input) {
  user-select: text;
}

:deep(.n-input-wrapper) {
  user-select: text;
}
</style>
