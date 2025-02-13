<script setup lang="ts">
import { NButton } from 'naive-ui'
import { SunnySharp, Moon } from '@vicons/ionicons5'
import { useTheme } from '../composables/theme'

const { isDarkMode, toggleTheme } = useTheme()
</script>

<template>
  <n-button circle secondary @click="toggleTheme">
    <template #icon>
      <component :is="isDarkMode ? SunnySharp : Moon" />
    </template>
  </n-button>
</template> 