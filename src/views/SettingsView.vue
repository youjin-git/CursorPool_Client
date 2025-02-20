<script setup lang="ts">
import { ref } from 'vue'
import { 
  NCard, 
  NSpace, 
  NForm, 
  NFormItem, 
  NInput, 
  NButton,
  NInputGroup,
  useMessage
} from 'naive-ui'
import { useRouter } from 'vue-router'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import LanguageSwitch from '../components/LanguageSwitch.vue'
import { changePassword, activate } from '@/api'
import { addHistoryRecord } from '../utils/history'

const router = useRouter()
const message = useMessage()
const { currentLang } = useI18n()

interface SettingsForm {
  activationCode: string
  currentPassword: string
  newPassword: string
  confirmPassword: string
}

const formValue = ref<SettingsForm>({
  activationCode: '',
  currentPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const loading = ref(false)

const handleActivate = async () => {
  if (!formValue.value.activationCode) {
    message.warning(messages[currentLang.value].message.pleaseInputActivationCode)
    return
  }

  loading.value = true
  try {
    const apiKey = localStorage.getItem('apiKey')
    if (!apiKey) {
      throw new Error('未找到 API Key')
    }

    await activate(apiKey, formValue.value.activationCode)
    message.success(messages[currentLang.value].message.activationSuccess)
    addHistoryRecord(
      '激活码兑换',
      '成功兑换激活码'
    )
    formValue.value.activationCode = ''
  } catch (error) {
    console.error('激活失败:', error)
    message.error(messages[currentLang.value].message.activationFailed)
  } finally {
    loading.value = false
  }
}

const handlePasswordChange = async () => {
  if (!formValue.value.currentPassword || !formValue.value.newPassword || !formValue.value.confirmPassword) {
    message.warning(messages[currentLang.value].message.pleaseInputPassword)
    return
  }
  if (formValue.value.newPassword !== formValue.value.confirmPassword) {
    message.error(messages[currentLang.value].message.passwordNotMatch)
    return
  }

  loading.value = true
  try {
    const apiKey = localStorage.getItem('apiKey')
    if (!apiKey) {
      throw new Error('未找到 API Key')
    }

    await changePassword(apiKey, formValue.value.currentPassword, formValue.value.newPassword)
    message.success(messages[currentLang.value].message.passwordChangeSuccess)
    addHistoryRecord(
      '密码修改',
      '成功修改密码'
    )
    formValue.value.currentPassword = ''
    formValue.value.newPassword = ''
    formValue.value.confirmPassword = ''
    
    await handleLogout()
  } catch (error) {
    message.error(messages[currentLang.value].message.passwordChangeFailed)
  } finally {
    loading.value = false
  }
}

const handleLogout = async () => {
  localStorage.removeItem('apiKey')
  await router.push('/dashboard')
  window.dispatchEvent(new CustomEvent('refresh_dashboard_data'))
  window.location.reload()
}
</script>

<template>
  <n-space vertical :size="24">
    <n-card title="Language / 语言">
      <n-space vertical>
        <language-switch />
      </n-space>
    </n-card>

    <n-card :title="messages[currentLang].settings.activation">
      <n-form
        :model="formValue"
        label-placement="left"
        label-width="120"
        require-mark-placement="right-hanging"
      >
        <n-form-item
          :label="messages[currentLang].settings.activationCode"
          path="activationCode"
        >
          <n-input-group style="width: 360px">
            <n-input
              v-model:value="formValue.activationCode"
              :placeholder="messages[currentLang].settings.activationCode"
              size="large"
            />
            <n-button
              type="primary"
              @click="handleActivate"
              :loading="loading"
              size="large"
            >
              {{ messages[currentLang].settings.activate }}
            </n-button>
          </n-input-group>
        </n-form-item>

        <n-form-item>
          <div style="padding-left: 40px">
            <n-button
              type="error"
              @click="handleLogout"
              size="large"
            >
              登出账户
            </n-button>
          </div>
        </n-form-item>
      </n-form>
    </n-card>

    <n-card :title="messages[currentLang].settings.changePassword">
      <n-form
        :model="formValue"
        label-placement="left"
        label-width="100"
        require-mark-placement="right-hanging"
      >
        <n-form-item :label="messages[currentLang].settings.currentPassword">
          <n-input
            v-model:value="formValue.currentPassword"
            type="password"
            show-password-on="click"
            :placeholder="messages[currentLang].settings.currentPassword"
          />
        </n-form-item>

        <n-form-item :label="messages[currentLang].settings.newPassword">
          <n-input
            v-model:value="formValue.newPassword"
            type="password"
            show-password-on="click"
            :placeholder="messages[currentLang].settings.newPassword"
          />
        </n-form-item>

        <n-form-item :label="messages[currentLang].settings.confirmPassword">
          <n-input
            v-model:value="formValue.confirmPassword"
            type="password"
            show-password-on="click"
            :placeholder="messages[currentLang].settings.confirmPassword"
          />
        </n-form-item>

        <div style="margin-top: 24px">
          <n-button type="primary" @click="handlePasswordChange">
            {{ messages[currentLang].settings.changePassword }}
          </n-button>
        </div>
      </n-form>
    </n-card>

    <n-card :title="messages[currentLang].settings.about">
      <p>Cursor Pool v0.1.0</p>
      <p> 2024 All Rights Reserved</p>
    </n-card>
  </n-space>
</template>