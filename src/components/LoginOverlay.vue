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
  NAutoComplete,
  NTag
} from 'naive-ui'
import { checkUser, sendCode, login } from '../api'
import type { LoginRequest } from '../api/types'
import type { SelectOption } from 'naive-ui'
import { h } from 'vue'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import type { HTMLAttributes } from 'vue'

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

// 邮箱提供商配置
const emailProviders = [
  {
    label: 'Google',
    domain: 'gmail.com',
    color: 'error'
  },
  {
    label: '腾讯',
    domain: 'qq.com',
    color: 'success'
  },
  {
    label: '腾讯',
    domain: 'foxmail.com',
    color: 'success'
  },
  {
    label: '网易',
    domain: '163.com',
    color: 'warning'
  },
  {
    label: 'Microsoft',
    domain: 'outlook.com',
    color: 'info'
  }
]

// 渲染邮箱选项标签
const renderLabel = (option: SelectOption) => {
  const domain = option.value?.toString().split('@')[1]
  const provider = emailProviders.find(p => p.domain === domain)
  
  return [
    option.label as string,
    ' ',
    h(NTag, {
      size: 'small',
      type: (provider?.color || 'default') as 'error' | 'success' | 'warning' | 'info' | 'default' | 'primary'
    }, { default: () => provider?.label || '邮箱' })
  ]
}

// 添加邮箱输入状态
const emailInputStatus = computed(() => {
  const email = formData.value.username
  if (!email) return undefined
  if (!emailRegex.test(email)) return 'error'
  const domain = email.split('@')[1]
  if (domain && !emailProviders.some(p => p.domain === domain)) return 'warning'
  return undefined
})

// 添加邮箱输入状态和提示信息
const emailInputFeedback = computed(() => {
  const email = formData.value.username
  if (!email) return ''
  if (!emailRegex.test(email)) {
    return messages[currentLang.value].login.emailInvalid
  }
  const domain = email.split('@')[1]
  if (domain && !emailProviders.some(p => p.domain === domain)) {
    return messages[currentLang.value].login.emailUnsupported
  }
  return ''
})

// 修改邮箱自动完成选项
const emailOptions = computed(() => {
  const inputValue = formData.value.username
  const atIndex = inputValue.lastIndexOf('@')
  
  // 只有当用户输入@后才显示选项
  if (atIndex === -1) return []
  
  const username = inputValue.substring(0, atIndex)
  if (!username) return []
  
  return emailProviders.map(provider => ({
    label: `${username}@${provider.domain}`,
    value: `${username}@${provider.domain}`
  }))
})

// 处理邮箱选择
function handleEmailSelect(value: string) {
  if (value && isValidEmail(value)) {
    formData.value.username = value
  }
}

// 验证邮箱格式
function isValidEmail(email: string): boolean {
  if (!emailRegex.test(email)) return false
  const domain = email.split('@')[1]
  return emailProviders.some(provider => provider.domain === domain)
}

// 检查用户是否存在的防抖定时器
let checkUserTimer: number | null = null

// 添加注册模式状态
const isRegisterMode = ref(false)

// 计算标题
const formTitle = computed(() => messages[currentLang.value].login[isRegisterMode.value ? 'registerButton' : 'title'])

// 计算按钮文本
const buttonText = computed(() => messages[currentLang.value].login[isRegisterMode.value ? 'registerButton' : 'loginButton'])

// 切换模式
function toggleMode() {
  isRegisterMode.value = !isRegisterMode.value
  // 清空表单
  formData.value = {
    username: '',
    password: '',
    sms_code: '',
  }
  // 注册模式下直接显示验证码框
  showVerifyCode.value = isRegisterMode.value
}

// 修改监听用户名变化的逻辑
watch(() => formData.value.username, async (newValue) => {
  if (!newValue || !isValidEmail(newValue)) {
    // 注册模式下保持验证码框显示
    showVerifyCode.value = isRegisterMode.value
    return
  }
  
  // 防抖处理
  if (checkUserTimer) clearTimeout(checkUserTimer)
  checkUserTimer = setTimeout(async () => {
    try {
      const result = await checkUser(newValue)
      
      // 如果是注册模式,当用户存在时自动切换到登录
      if (isRegisterMode.value && result.exists) {
        message.info(messages[currentLang.value].login.userExists)
        isRegisterMode.value = false
        showVerifyCode.value = result.needCode
      }
      // 如果是登录模式,当用户不存在时只提示
      else if (!isRegisterMode.value && !result.exists) {
        message.error(messages[currentLang.value].login.userNotExists)
        showVerifyCode.value = false
      } 
      // 其他情况
      else {
        // 注册模式始终显示验证码,登录模式根据needCode决定
        showVerifyCode.value = isRegisterMode.value || result.needCode
      }
    } catch (error) {
      console.error('Check user failed:', error)
      // 发生错误时,注册模式下保持验证码框显示
      showVerifyCode.value = isRegisterMode.value
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
    countDown.value = result.expireIn
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

// 添加密码验证正则
const passwordRegex = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,}$/

// 添加密码输入状态
const passwordInputStatus = computed(() => {
  const password = formData.value.password
  if (!password) return undefined
  if (!passwordRegex.test(password)) return 'error'
  return undefined
})

// 添加密码输入状态和提示信息
const passwordInputFeedback = computed(() => {
  const password = formData.value.password
  if (!password) return ''
  if (!passwordRegex.test(password)) {
    return messages[currentLang.value].login.passwordInvalid
  }
  return ''
})

// 修改处理提交的逻辑，添加密码验证
async function handleSubmit() {
  if (!formData.value.username || !isValidEmail(formData.value.username)) {
    message.error(messages[currentLang.value].login.emailError)
    return
  }

  if (!formData.value.password || !passwordRegex.test(formData.value.password)) {
    message.error(messages[currentLang.value].login.passwordInvalid)
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
    if (result.apiKey) {
      localStorage.setItem('apiKey', result.apiKey)
      message.success(messages[currentLang.value].login.loginSuccess)
      emit('login-success')
    } else {
      message.error(messages[currentLang.value].login.loginFailed)
    }
  } catch (error) {
    message.error(messages[currentLang.value].login.loginFailed + ': ' + (error instanceof Error ? error.message : ''))
  } finally {
    loading.value = false
  }
}

const emit = defineEmits(['login-success'])

const { currentLang } = useI18n()

// 定义自定义输入属性类型
interface CustomInputProps extends HTMLAttributes {
  autocomplete?: string
  'data-form-type'?: string
  'data-lpignore'?: string
}

// 定义输入属性
const inputProps = {
  autocomplete: 'off',
  'data-form-type': 'other',
  'data-lpignore': 'true'
} as CustomInputProps
</script>

<template>
  <!-- 添加一个隐藏的假表单来欺骗浏览器的自动填充 -->
  <form style="display: none" aria-hidden="true">
    <input type="text" />
    <input type="email" />
    <input type="password" />
  </form>

  <div class="login-overlay">
    <n-card :title="formTitle" class="login-card">
      <n-form>
        <n-form-item :label="messages[currentLang].login.emailPlaceholder">
          <n-auto-complete
            v-model:value="formData.username"
            :options="emailOptions"
            :status="emailInputStatus"
            :placeholder="messages[currentLang].login.emailPlaceholder"
            :render-label="renderLabel"
            :disabled="loading"
            @select="handleEmailSelect"
            :clear-after-select="false"
            autocomplete="off"
            :input-props="inputProps"
          />
          <template #feedback>
            {{ emailInputFeedback }}
          </template>
        </n-form-item>
        
        <n-form-item 
          :label="messages[currentLang].login.passwordPlaceholder"
          :status="passwordInputStatus"
        >
          <n-input 
            v-model:value="formData.password"
            type="password"
            :placeholder="messages[currentLang].login.passwordPlaceholder"
            :disabled="loading"
          />
          <template #feedback>
            {{ passwordInputFeedback }}
          </template>
        </n-form-item>

        <n-form-item v-if="showVerifyCode" :label="messages[currentLang].login.smsCodePlaceholder">
          <n-space>
            <n-input 
              v-model:value="formData.sms_code"
              :placeholder="messages[currentLang].login.smsCodePlaceholder"
              :disabled="loading"
            />
            <n-button 
              :disabled="loading || countDown > 0 || !isValidEmail(formData.username)"
              @click="handleSendCode"
              secondary
            >
              {{ countDown > 0 ? messages[currentLang].login.resendCode.replace('{seconds}', countDown.toString()) : messages[currentLang].login.sendCode }}
            </n-button>
          </n-space>
        </n-form-item>

        <n-space vertical :size="12">
          <n-button 
            type="primary" 
            block 
            @click="handleSubmit"
            :loading="loading"
            :disabled="!isValidEmail(formData.username)"
          >
            {{ buttonText }}
          </n-button>

          <n-button 
            quaternary 
            block
            @click="toggleMode"
            :disabled="loading"
          >
            {{ isRegisterMode 
              ? messages[currentLang].login.hasAccount 
              : `${messages[currentLang].login.noAccount} ${messages[currentLang].login.register}`
            }}
          </n-button>
        </n-space>
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

:deep(.n-form-item-feedback-wrapper) {
  min-height: 20px;
}

:deep(.n-form-item-feedback) {
  color: var(--n-feedback-text-color);
  font-size: 12px;
}

/* 添加以下样式来进一步防止自动填充 */
:deep(.n-input__input-el) {
  /* 禁用 webkit 浏览器的自动填充样式 */
  &:-webkit-autofill,
  &:-webkit-autofill:hover,
  &:-webkit-autofill:focus,
  &:-webkit-autofill:active {
    -webkit-box-shadow: 0 0 0 30px var(--n-color) inset !important;
    -webkit-text-fill-color: var(--n-text-color) !important;
    transition: background-color 5000s ease-in-out 0s;
  }
}
</style>
