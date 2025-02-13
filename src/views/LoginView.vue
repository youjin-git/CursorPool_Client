<script setup lang="ts">
import { ref, watch } from 'vue'
import { 
  NCard, 
  NForm, 
  NFormItem, 
  NInput, 
  NButton, 
  useMessage,
  NSpace
} from 'naive-ui'
import { useRouter } from 'vue-router'

const router = useRouter()
const message = useMessage()

interface FormState {
  email: string
  password: string
  verifyCode?: string
}

const formRef = ref<typeof NForm | null>(null)
const loading = ref(false)
const isExistingUser = ref(false)
const formValue = ref<FormState>({
  email: '',
  password: '',
})

// 检查用户是否存在的防抖函数
let checkTimeout: NodeJS.Timeout
const checkUserExistence = async (email: string) => {
  if (!email) return
  
  clearTimeout(checkTimeout)
  checkTimeout = setTimeout(async () => {
    loading.value = true
    try {
      // TODO: 实际检查用户是否存在的API调用
      const exists = false // await api.checkUser(email)
      isExistingUser.value = exists
    } catch (error) {
      message.error('检查用户状态失败')
    } finally {
      loading.value = false
    }
  }, 500)
}

// 监听邮箱变化
watch(() => formValue.value.email, (newValue) => {
  checkUserExistence(newValue)
})

const handleSubmit = async () => {
  try {
    loading.value = true
    if (isExistingUser.value) {
      // TODO: 登录逻辑
      message.success('登录成功')
    } else {
      // TODO: 注册逻辑
      message.success('注册成功')
    }
    router.push('/')
  } catch (error) {
    message.error(isExistingUser.value ? '登录失败' : '注册失败')
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-container">
    <n-card :title="isExistingUser ? '登录' : '注册'" class="login-card">
      <n-form
        ref="formRef"
        :model="formValue"
        label-placement="left"
        label-width="80"
        require-mark-placement="right-hanging"
      >
        <n-form-item label="邮箱" path="email">
          <n-input
            v-model:value="formValue.email"
            placeholder="请输入邮箱"
            :disabled="loading"
          />
        </n-form-item>
        
        <n-form-item label="密码" path="password">
          <n-input
            v-model:value="formValue.password"
            type="password"
            placeholder="请输入密码"
            :disabled="loading"
          />
        </n-form-item>

        <n-form-item
          v-if="!isExistingUser"
          label="验证码"
          path="verifyCode"
        >
          <n-space>
            <n-input
              v-model:value="formValue.verifyCode"
              placeholder="请输入验证码"
              :disabled="loading"
            />
            <n-button
              :disabled="!formValue.email || loading"
              @click="() => message.info('验证码已发送')"
            >
              获取验证码
            </n-button>
          </n-space>
        </n-form-item>

        <div style="margin-top: 24px">
          <n-button
            type="primary"
            block
            @click="handleSubmit"
            :loading="loading"
          >
            {{ isExistingUser ? '登录' : '注册' }}
          </n-button>
        </div>
      </n-form>
    </n-card>
  </div>
</template>

<style scoped>
.login-container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f0f2f5;
}

.login-card {
  width: 100%;
  max-width: 400px;
}
</style> 