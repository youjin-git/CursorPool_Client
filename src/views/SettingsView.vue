<script setup lang="ts">
import { ref, onMounted } from 'vue'
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
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import LanguageSwitch from '../components/LanguageSwitch.vue'
import CursorRunningModal from '../components/CursorRunningModal.vue'
import { 
  changePassword, 
  activate, 
  checkCursorRunning,
  checkHookStatus,
  applyHook,
  restoreHook
} from '@/api'
import { addHistoryRecord } from '../utils/history'
import { version } from '../../package.json'
import { useUserStore } from '../stores/user'

const message = useMessage()
const { currentLang, i18n } = useI18n()
const userStore = useUserStore()

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

// 修改控制状态
const controlStatus = ref({
  isHooked: false
})

// 为每个操作添加单独的加载状态
const applyHookLoading = ref(false)
const restoreHookLoading = ref(false)

const showControlRunningModal = ref(false)
const pendingControlAction = ref<'applyHook' | 'restoreHook' | null>(null)

// 为激活和修改密码添加独立的加载状态
const activateLoading = ref(false)
const passwordChangeLoading = ref(false)

const handleActivate = async () => {
  if (!formValue.value.activationCode) {
    message.error('请输入激活码')
    return
  }
  
  activateLoading.value = true
  try {
    await activate(formValue.value.activationCode)
    message.success('激活成功')
    addHistoryRecord(
      '激活码兑换',
      '成功兑换激活码'
    )
    formValue.value.activationCode = ''
  } catch (error) {
    message.error(error instanceof Error ? error.message : '激活失败')
  } finally {
    activateLoading.value = false
  }
}

const handleChangePassword = async () => {
  if (!formValue.value.currentPassword || !formValue.value.newPassword || !formValue.value.confirmPassword) {
    message.error('请填写完整密码信息')
    return
  }
  
  if (formValue.value.newPassword !== formValue.value.confirmPassword) {
    message.error('两次输入的新密码不一致')
    return
  }
  
  passwordChangeLoading.value = true
  try {
    await changePassword(
      formValue.value.currentPassword,
      formValue.value.newPassword
    )
    message.success('密码修改成功')
    
    formValue.value.currentPassword = ''
    formValue.value.newPassword = ''
    formValue.value.confirmPassword = ''
    
    await handleLogout()
  } catch (error) {
    message.error(error instanceof Error ? error.message : '密码修改失败')
  } finally {
    passwordChangeLoading.value = false
  }
}

const handleLogout = async () => {
  try {
    await userStore.logout()
    addHistoryRecord('用户操作', '用户登出')
  } catch (error) {
    message.error('登出失败')
  }
}

// 检查控制状态
const checkControlStatus = async () => {
  try {
    controlStatus.value.isHooked = await checkHookStatus()
  } catch (error) {
    console.error('检查控制状态失败:', error)
  }
}

// 修改 handleControlAction 函数
const handleControlAction = async (action: 'applyHook' | 'restoreHook', force_kill: boolean = false) => {
  // 根据操作设置对应的加载状态
  const loadingRef = {
    'applyHook': applyHookLoading,
    'restoreHook': restoreHookLoading
  }[action]

  try {
    loadingRef.value = true
    if (!force_kill) {
      const isRunning = await checkCursorRunning()
      if (isRunning) {
        showControlRunningModal.value = true
        pendingControlAction.value = action
        return
      }
    }

    let successMessage = ''
    let historyAction = ''
    
    switch (action) {
      case 'applyHook':
        await applyHook(force_kill)
        successMessage = messages[currentLang.value].systemControl.messages.applyHookSuccess
        historyAction = messages[currentLang.value].systemControl.history.applyHook
        controlStatus.value.isHooked = true
        break
      case 'restoreHook':
        await restoreHook(force_kill)
        successMessage = messages[currentLang.value].systemControl.messages.restoreHookSuccess
        historyAction = messages[currentLang.value].systemControl.history.restoreHook
        controlStatus.value.isHooked = false
        break
    }

    message.success(successMessage)
    showControlRunningModal.value = false
    addHistoryRecord('系统控制', historyAction)
  } catch (error) {
    message.error(error instanceof Error ? error.message : '操作失败')
  } finally {
    loadingRef.value = false
  }
}

// 处理强制关闭
const handleControlForceKill = async () => {
  if (pendingControlAction.value) {
    await handleControlAction(pendingControlAction.value, true)
  }
}

// 在组件挂载时检查控制状态
onMounted(async () => {
  await checkControlStatus()
})
</script>

<template>
  <n-space vertical :size="24">
    <n-card :title="i18n.systemControl.title">
      <n-space vertical>
        <!-- Hook 控制部分 -->
        <n-space justify="space-between" align="center">
          <span>{{ i18n.systemControl.hookStatus }}: {{ controlStatus.isHooked ? i18n.systemControl.hookApplied : i18n.systemControl.hookNotApplied }}</span>
          <n-space>
            <n-button 
              type="warning" 
              :loading="applyHookLoading"
              :disabled="controlStatus.isHooked"
              @click="handleControlAction('applyHook')"
              style="width: 120px"
            >
              {{ i18n.systemControl.applyHook }}
            </n-button>
            <n-button 
              type="primary"
              :loading="restoreHookLoading"
              :disabled="!controlStatus.isHooked"
              @click="handleControlAction('restoreHook')"
              style="width: 120px"
            >
              {{ i18n.systemControl.restoreHook }}
            </n-button>
          </n-space>
        </n-space>
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
              :loading="activateLoading"
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
              {{ i18n.common.logout }}
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
            maxlength="20"
            minlength="6"
            :allow-input="(value) => /^[a-zA-Z0-9]*$/.test(value)"
          />
        </n-form-item>

        <n-form-item :label="messages[currentLang].settings.newPassword">
          <n-input
            v-model:value="formValue.newPassword"
            type="password"
            show-password-on="click"
            :placeholder="messages[currentLang].settings.newPassword"
            maxlength="20"
            minlength="6"
            :allow-input="(value) => /^[a-zA-Z0-9]*$/.test(value)"
          />
        </n-form-item>

        <n-form-item :label="messages[currentLang].settings.confirmPassword">
          <n-input
            v-model:value="formValue.confirmPassword"
            type="password"
            show-password-on="click"
            :placeholder="messages[currentLang].settings.confirmPassword"
            maxlength="20"
            minlength="6"
            :allow-input="(value) => /^[a-zA-Z0-9]*$/.test(value)"
          />
        </n-form-item>

        <div style="margin-top: 24px">
          <n-button 
            type="primary" 
            @click="handleChangePassword"
            :loading="passwordChangeLoading"
          >
            {{ messages[currentLang].settings.changePassword }}
          </n-button>
        </div>
      </n-form>
    </n-card>

    <n-card title="Language / 语言">
      <n-space vertical>
        <language-switch />
      </n-space>
    </n-card>

    <n-card :title="messages[currentLang].settings.about">
      <n-space vertical :size="12">
        <p>{{ i18n.about.appName }} v{{ version }}</p>
        <p>
          {{ i18n.about.copyright }} © {{ new Date().getFullYear() }} 
          <n-button text tag="a" href="https://github.com/Sanyela" target="_blank">Sanyela</n-button> & 
          <n-button text tag="a" href="https://github.com/Cloxl" target="_blank">Cloxl</n-button>
        </p>
        <p>{{ i18n.about.license }}</p>
      </n-space>
    </n-card>

    <cursor-running-modal
      v-model:show="showControlRunningModal"
      :title="i18n.common.cursorRunning"
      :content="i18n.common.cursorRunningMessage"
      :confirm-button-text="i18n.common.forceClose"
      @confirm="handleControlForceKill"
    />
  </n-space>
</template>