<script setup lang="ts">
import { NButton, NDropdown } from 'naive-ui'
import { useI18n } from '../locales'
import { locales, type Language } from '../locales'
import { h } from 'vue'

const { currentLang, setLanguage } = useI18n()

const options = Object.entries(locales).map(([key, value]) => ({
  label: value.name,
  key
}))

const handleSelect = (key: string) => {
  setLanguage(key as Language)
}
</script>

<template>
  <n-dropdown :options="options" @select="handleSelect" placement="bottom-start">
    <n-button quaternary style="min-width: 100px">
      {{ locales[currentLang].name }}
    </n-button>
  </n-dropdown>
</template>

<style scoped>
.n-button {
  text-align: left;
}
</style> 