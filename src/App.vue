<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NGlobalStyle } from 'naive-ui'
import { useTheme } from './composables/theme'
import { themeOverrides } from './styles/theme'
import { useI18n, initLanguage } from './locales'
import { locales } from './locales'
import { computed, onMounted } from 'vue'
import { useHistoryStore } from './stores/history'

const { currentTheme } = useTheme()
const { currentLang } = useI18n()
const historyStore = useHistoryStore()

const locale = computed(() => locales[currentLang.value].locale)
const dateLocale = computed(() => locales[currentLang.value].dateLocale)

// 应用启动时初始化
onMounted(async () => {
  // 初始化语言设置
  await initLanguage()
  
  // 使用统一的初始化方法
  await historyStore.init()
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