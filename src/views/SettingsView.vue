<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { 
  NCard, 
  NSpace, 
  NForm, 
  NFormItem, 
  NInput, 
  NButton,
  useMessage
} from 'naive-ui'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import LanguageSwitch from '../components/LanguageSwitch.vue'
import { request } from '@/api'
import { UserInfoResponse } from '@/api/types'

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

const userInfo = ref<UserInfoResponse['data']>()

const fetchUserInfo = async () => {
  try {
    const response = await request.get<UserInfoResponse>('/user/info')
    if (response.status === 'success') {
      userInfo.value = response.data
    }
  } catch (error) {
    console.error('获取用户信息失败:', error)
  }
}

onMounted(() => {
  fetchUserInfo()
})

const handleActivate = () => {
  if (!formValue.value.activationCode) {
    message.warning(messages[currentLang.value].message.pleaseInputActivationCode)
    return
  }
  // TODO: 激活码兑换逻辑
  message.success(messages[currentLang.value].message.activationSuccess)
  formValue.value.activationCode = ''
}

const handlePasswordChange = () => {
  if (!formValue.value.currentPassword || !formValue.value.newPassword || !formValue.value.confirmPassword) {
    message.warning(messages[currentLang.value].message.pleaseInputPassword)
    return
  }
  if (formValue.value.newPassword !== formValue.value.confirmPassword) {
    message.error(messages[currentLang.value].message.passwordNotMatch)
    return
  }
  // TODO: 修改密码逻辑
  message.success(messages[currentLang.value].message.passwordChangeSuccess)
  formValue.value.currentPassword = ''
  formValue.value.newPassword = ''
  formValue.value.confirmPassword = ''
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
        label-width="100"
        require-mark-placement="right-hanging"
      >
        <n-form-item :label="messages[currentLang].settings.activationCode">
          <n-space>
            <n-input
              v-model:value="formValue.activationCode"
              :placeholder="messages[currentLang].settings.activationCode"
              style="width: 240px"
            >
              <template #suffix>
                <n-button
                  type="primary"
                  secondary
                  size="small"
                  @click="handleActivate"
                >
                  {{ messages[currentLang].settings.activate }}
                </n-button>
              </template>
            </n-input>
          </n-space>
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
      <p>© 2024 All Rights Reserved</p>
    </n-card>
  </n-space>
</template> 