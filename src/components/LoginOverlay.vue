<script setup lang="ts">
  import { ref, computed } from 'vue'
  import { NCard, NForm, NFormItem, NInput, NButton, useMessage, NSpace } from 'naive-ui'
  import { saveUserApiToken } from '../api'
  import { useI18n } from '../locales'
  import { messages } from '../locales/messages'
  import { addHistoryRecord } from '../utils/history'
  import { useUserStore } from '../stores/user'
  import InboundSelector from './InboundSelector.vue'

  // 定义组件事件
  const emit = defineEmits(['login-success'])

  // 基础状态
  const message = useMessage()
  const { currentLang, t } = useI18n()
  const userStore = useUserStore()

  // 激活码状态
  const activationCode = ref('')
  const activationLoading = ref(false)

  /**
   * 处理激活码登录
   */
  async function handleActivation() {
    if (!activationCode.value) {
      message.error(currentLang.value === 'zh-CN' ? '请输入激活码' : 'Please enter activation code')
      return
    }

    try {
      activationLoading.value = true

      // 直接将激活码作为API Token保存
      await saveUserApiToken(activationCode.value)

      // 在localStorage中也保存一份，用于路由守卫判断
      localStorage.setItem('apiKey', activationCode.value)

      // 记录激活成功
      addHistoryRecord(
        currentLang.value === 'zh-CN' ? '激活' : 'Activation',
        currentLang.value === 'zh-CN' ? `激活码登录成功` : `Activation code login successful`,
      )

      // 显示成功消息
      message.success(currentLang.value === 'zh-CN' ? '激活成功' : 'Activation successful')

      // 触发登录成功事件
      emit('login-success')
    } catch (error) {
      message.error(
        error instanceof Error
          ? error.message
          : currentLang.value === 'zh-CN'
            ? '激活失败'
            : 'Activation failed',
      )
    } finally {
      activationLoading.value = false
    }
  }
</script>

<template>
  <!-- 添加一个隐藏的假表单来欺骗浏览器的自动填充 -->
  <form style="display: none" aria-hidden="true">
    <input type="text" />
    <input type="email" />
    <input type="password" />
  </form>

  <div class="login-overlay">
    <n-card
      :title="currentLang === 'zh-CN' ? '激活码登录' : 'Activation Code Login'"
      class="login-card"
    >
      <n-form class="compact-form">
        <n-form-item>
          <n-input
            v-model:value="activationCode"
            :placeholder="currentLang === 'zh-CN' ? '请输入激活码' : 'Enter activation code'"
            maxlength="50"
            @keyup.enter="handleActivation"
          />
        </n-form-item>

        <n-space justify="center">
          <n-button
            type="primary"
            block
            :loading="activationLoading"
            class="login-button"
            @click="handleActivation"
          >
            {{ currentLang === 'zh-CN' ? '激活并登录' : 'Activate and Login' }}
          </n-button>
          <!-- <inbound-selector compact :show-label="false" /> -->
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
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(8px);
    z-index: 1000;
    user-select: none;
  }

  .login-card {
    width: 360px;
    max-width: 90%;
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
  }

  /* 标签页容器样式 */
  .full-width-tabs :deep(.n-tabs-wrapper) {
    display: flex;
    width: 100%;
  }

  /* 标签页包装器样式 */
  .full-width-tabs :deep(.n-tabs-tab-wrapper) {
    flex: 1;
    display: flex;
  }

  /* 标签页样式 */
  .full-width-tabs :deep(.n-tabs-tab) {
    flex: 1;
    display: flex;
    justify-content: center;
    font-size: 16px;
    padding: 0;
    margin: 0;
  }

  /* 激活的标签页样式 */
  .full-width-tabs :deep(.n-tabs-tab.n-tabs-tab--active) {
    font-weight: bold;
  }

  /* 隐藏标签页内的padding元素 */
  .full-width-tabs :deep(.n-tabs-tab-pad) {
    display: none !important;
    width: 0 !important;
  }

  /* 标签页内容区域样式 */
  .full-width-tabs :deep(.n-tabs-tab-pane) {
    padding-top: 12px;
  }

  /* 标签页下划线样式 */
  .full-width-tabs :deep(.n-tabs-bar) {
    width: 50% !important;
    transition: transform 0.3s var(--n-bezier);
  }

  /* 缩小输入框间距 */
  .compact-form :deep(.n-form-item) {
    margin-bottom: 5px;
  }

  .compact-form :deep(.n-form-item:last-child) {
    margin-bottom: 0;
  }

  /* 发送验证码按钮样式 */
  .send-code-btn {
    min-width: 100px;
    font-size: 13px;
    padding: 0 8px;
  }

  :deep(.n-card) {
    background: var(--n-color);
    color: var(--n-text-color);
  }

  :deep(.n-card-header) {
    text-align: center;
    font-size: 1.5em;
    padding-top: 14px;
    padding-bottom: 4px;
  }

  :deep(.n-card__content) {
    padding-top: 4px;
    padding-bottom: 14px;
  }

  :deep(.n-input) {
    user-select: text;
  }

  :deep(.n-input-wrapper) {
    user-select: text;
  }

  :deep(.n-form-item-feedback-wrapper) {
    min-height: 12px;
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

  /* 自定义错误状态样式 */
  :deep(.n-auto-complete.n-auto-complete--status-error .n-input) {
    border-color: #ff4d4f;
  }

  :deep(.n-auto-complete.n-auto-complete--status-error:hover .n-input) {
    border-color: #ff7875;
  }

  :deep(.n-auto-complete.n-auto-complete--status-error:focus .n-input) {
    border-color: #ff7875;
    box-shadow: 0 0 0 2px rgba(255, 77, 79, 0.2);
  }

  /* 登录操作按钮区域 */
  .login-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }
</style>
