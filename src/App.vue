<script setup lang="ts">
import { NConfigProvider, NMessageProvider, NGlobalStyle } from 'naive-ui'
import { useTheme } from './composables/theme'
import { themeOverrides } from './styles/theme'

const { currentTheme } = useTheme()
</script>

<template>
  <n-config-provider :theme="currentTheme" :theme-overrides="themeOverrides">
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