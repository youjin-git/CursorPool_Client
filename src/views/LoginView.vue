<script setup lang="ts">
import { ref } from 'vue'
import { 
  NCard, 
  NForm, 
  NFormItem, 
  NInput, 
  NButton, 
  useMessage
} from 'naive-ui'
import { useRouter } from 'vue-router'
import type { Router } from 'vue-router'
import { LoginResponse, ApiResponse } from '@/api/types'
import { invoke } from '@tauri-apps/api/core'

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
    const response = await invoke<ApiResponse<LoginResponse>>('login', {
      username: formValue.value.username,
      password: formValue.value.password
    })
    
    if (response.status === 'success' && response.data?.api_key) {
      localStorage.setItem('api_key', response.data.api_key)
      localStorage.setItem('username', formValue.value.username)
      message.success('登录成功')
      
      // 发送事件通知 DashboardView 刷新数据
      window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
      
      router.push('/')
    } else {
      throw new Error(response.message || '登录失败')
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
}

.login-card {
  width: 100%;
  max-width: 400px;
}

:deep(.n-card) {
  background: var(--n-color);
}
</style>