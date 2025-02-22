<script setup lang="ts">
import { NButton, NIcon } from 'naive-ui'
import { SunnySharp, Moon } from '@vicons/ionicons5'
import { useTheme } from '../composables/theme'

const { isDarkMode, toggleTheme } = useTheme()
</script>

<template>
  <n-button circle secondary @click="toggleTheme">
    <template #icon>
      <n-icon>
        <component :is="isDarkMode ? SunnySharp : Moon" />
      </n-icon>
    </template>
  </n-button>
</template> 