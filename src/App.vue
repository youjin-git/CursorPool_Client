<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NGlobalStyle } from 'naive-ui'
import { useTheme } from './composables/theme'
import { themeOverrides } from './styles/theme'
import { useI18n } from './locales'
import { locales } from './locales'
import { computed } from 'vue'

const { currentTheme } = useTheme()
const { currentLang } = useI18n()

const locale = computed(() => locales[currentLang.value].locale)
const dateLocale = computed(() => locales[currentLang.value].dateLocale)
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