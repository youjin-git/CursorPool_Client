<script setup lang="ts">
import { NLayout, NLayoutSider, NMenu, NIcon } from 'naive-ui'
import { ref, computed } from 'vue'
import type { Router } from 'vue-router'
import { useRouter } from 'vue-router'
import { 
  HomeSharp, 
  SettingsSharp,
  TimeSharp 
} from '@vicons/ionicons5'
import ThemeToggle from '../components/ThemeToggle.vue'
import { Component, h } from 'vue'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'

const router = useRouter() as unknown as Router
const { currentLang } = useI18n()

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions = computed(() => [
  {
    label: messages[currentLang.value].menu.dashboard,
    key: 'dashboard',
    icon: renderIcon(HomeSharp),
    path: '/'
  },
  {
    label: messages[currentLang.value].menu.history,
    key: 'history',
    icon: renderIcon(TimeSharp),
    path: '/history'
  },
  {
    label: messages[currentLang.value].menu.settings,
    key: 'settings',
    icon: renderIcon(SettingsSharp),
    path: '/settings'
  }
])

const handleMenuClick = (key: string) => {
  const menuItem = menuOptions.value.find(item => item.key === key)
  if (menuItem) {
    router.push(menuItem.path)
  }
}

const collapsed = ref(true)
const contentMarginLeft = computed(() => collapsed.value ? '64px' : '240px')
</script>

<template>
  <n-layout position="absolute">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="240"
      :collapsed="collapsed"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
      :native-scrollbar="false"
      position="absolute"
    >
      <div class="logo">
        <h2 v-if="!collapsed">Cursor Pool</h2>
        <h2 v-else>CP</h2>
      </div>
      <n-menu
        :options="menuOptions"
        :collapsed="collapsed"
        :collapsed-width="64"
        :collapsed-icon-size="22"
        :default-value="menuOptions[0].key"
        @update:value="handleMenuClick"
      />
      <div class="sider-footer">
        <theme-toggle />
      </div>
    </n-layout-sider>
    <n-layout 
      :native-scrollbar="false" 
      content-style="padding: 24px;"
      :style="{ marginLeft: contentMarginLeft }"
    >
      <router-view />
    </n-layout>
  </n-layout>
</template>

<style scoped>
.logo {
  padding: 16px;
  text-align: center;
}

.logo h2 {
  margin: 0;
  font-size: 1.25rem;
  white-space: nowrap;
}

.sider-footer {
  position: absolute;
  bottom: 16px;
  left: 0;
  right: 0;
  display: flex;
  justify-content: center;
  padding: 0 16px;
  z-index: 1;
}
</style> 