import { ref, watch } from 'vue'
import { darkTheme, lightTheme } from 'naive-ui'
import type { GlobalTheme } from 'naive-ui'

// 从 localStorage 读取初始主题
const savedTheme = localStorage.getItem('theme-mode')
export const isDarkMode = ref(savedTheme === 'dark')
export const currentTheme = ref<GlobalTheme | null>(isDarkMode.value ? darkTheme : null)

export function useTheme() {
  // 监听主题变化并保存到 localStorage
  watch(isDarkMode, (newValue) => {
    localStorage.setItem('theme-mode', newValue ? 'dark' : 'light')
  })

  const toggleTheme = () => {
    isDarkMode.value = !isDarkMode.value
    currentTheme.value = isDarkMode.value ? darkTheme : null
  }

  return {
    isDarkMode,
    currentTheme,
    toggleTheme
  }
} 