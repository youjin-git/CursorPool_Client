<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NGlobalStyle } from 'naive-ui'
import { useTheme } from './composables/theme'
import { themeOverrides } from './styles/theme'
import { useI18n } from './locales'
import { locales } from './locales'
import { computed, onMounted } from 'vue'
import { syncLocalHistoryToBackend } from './utils/history'
import { syncLocalAccountsToBackend } from './utils/historyAccounts'
import { useHistoryStore } from './stores/history'

const { currentTheme } = useTheme()
const { currentLang } = useI18n()
const historyStore = useHistoryStore()

const locale = computed(() => locales[currentLang.value].locale)
const dateLocale = computed(() => locales[currentLang.value].dateLocale)

// 应用启动时初始化
onMounted(async () => {
  // 同步本地历史记录到后端
  await syncLocalHistoryToBackend()
  
  // 同步本地历史账户到后端
  await syncLocalAccountsToBackend()
  
  // 加载历史记录
  await historyStore.loadHistoryRecords()
  
  // 设置历史记录更新监听器
  const removeListener = historyStore.setupHistoryListener()
  
  // 组件卸载时移除监听器
  return () => {
    removeListener()
  }
})
</script>

<template>
  <n-config-provider
    :theme="currentTheme"
    :theme-overrides="themeOverrides"
    :locale="locale"
    :date-locale="dateLocale"
  >
    <n-message-provider>
      <router-view />
      <n-global-style />
    </n-message-provider>
  </n-config-provider>
</template>

<style>
body {
  margin: 0;
  font-family: "JetBrains Mono", -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif;
}
</style>