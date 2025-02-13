<script setup lang="ts">
import { ref } from 'vue'
import { 
  NCard, 
  NForm, 
  NFormItem, 
  NInput, 
  NButton, 
  useMessage,
} from 'naive-ui'
import { useRouter } from 'vue-router'
import type { Router } from 'vue-router'
import { login } from '@/api'
import { LoginResponse } from '@/api/types'

const router = useRouter() as unknown as Router
const message = useMessage()

interface FormState {
  username: string
  password: string
}

const formRef = ref<typeof NForm | null>(null)
const loading = ref(false)
const isExistingUser = ref(false)
const formValue = ref<FormState>({
  username: '',
  password: ''
})

const handleSubmit = async () => {
  try {
    loading.value = true
    const response: LoginResponse = await login(formValue.value.username, formValue.value.password)
    
    if (response.status === 'success') {
      localStorage.setItem('token', response.api_key)
      message.success('登录成功')
      router.push('/')
    } else {
      throw new Error(response.message)
    }
  } catch (error) {
    const err = error as Error
    message.error(err.message || '登录失败')
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
        <n-form-item label="密码" path="password">
          <n-input
            v-model:value="formValue.password"
            type="password"
            placeholder="请输入密码"
            :disabled="loading"
          />
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