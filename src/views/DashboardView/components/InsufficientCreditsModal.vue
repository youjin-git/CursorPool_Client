<script setup lang="ts">
import { ref } from 'vue'
import { NModal, NSpace, NButton, NInput, NForm, NFormItem } from 'naive-ui'
import { useI18n } from '../../../locales'
import { activate } from '@/api'
import { addHistoryRecord } from '../../../utils/history'

defineProps<{
  show: boolean
  userCredits: number
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'activate-success'): void
}>()

const { i18n } = useI18n()
const activationCode = ref('')
const loading = ref(false)
const errorMessage = ref('')

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const handleActivate = async () => {
  if (!activationCode.value) {
    errorMessage.value = i18n.value.message.pleaseInputActivationCode
    return
  }

  try {
    loading.value = true
    errorMessage.value = ''
    
    // 获取API密钥
    const apiKey = localStorage.getItem('apiKey')
    if (!apiKey) {
      throw new Error('未找到API密钥，请重新登录')
    }
    
    // 调用激活API
    await activate(apiKey, activationCode.value)
    
    // 记录操作历史
    addHistoryRecord(
      '卡密激活',
      `激活卡密: ${activationCode.value.substring(0, 4)}****`
    )
    
    // 清空输入
    activationCode.value = ''
    
    // 通知父组件激活成功
    emit('activate-success')
    
    // 关闭模态窗口
    updateShow(false)
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '激活失败，请检查卡密是否正确'
  } finally {
    loading.value = false
  }
}

const handleCancel = () => {
  activationCode.value = ''
  errorMessage.value = ''
  updateShow(false)
}
</script>

<template>
  <n-modal
    :show="show"
    preset="dialog"
    title="积分不足"
    :closable="true"
    :mask-closable="false"
    @update:show="updateShow"
  >
    <template #default>
      <n-form>
        <p>您当前剩余积分不足，账户切换需要消耗50积分。</p>
        <p style="margin-top: 12px; color: #ff4d4f;">
          当前积分: {{ userCredits }}，还需要: {{ Math.max(0, 50 - userCredits) }} 积分
        </p>
        
        <n-form-item :label="i18n.message.pleaseInputActivationCode" style="margin-top: 16px;">
          <n-input 
            v-model:value="activationCode" 
            placeholder="请输入卡密" 
            :disabled="loading"
          />
        </n-form-item>
        
        <p v-if="errorMessage" style="color: #ff4d4f; margin-top: 8px;">
          {{ errorMessage }}
        </p>
      </n-form>
    </template>
    <template #action>
      <n-space justify="end">
        <n-button @click="handleCancel" :disabled="loading">
          取消
        </n-button>
        <n-button type="primary" @click="handleActivate" :loading="loading">
          激活卡密
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template> 