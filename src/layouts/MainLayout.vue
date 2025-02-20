<script setup lang="ts">
import { NLayout, NLayoutSider, NMenu, NIcon, NButton } from 'naive-ui'
import { ref, computed } from 'vue'
import type { Router } from 'vue-router'
import { useRouter } from 'vue-router'
import { 
  HomeSharp, 
  SettingsSharp,
  TimeSharp,
  Close,
  RemoveOutline
} from '@vicons/ionicons5'
import ThemeToggle from '../components/ThemeToggle.vue'
import LoginOverlay from '../components/LoginOverlay.vue'
import { Component, h } from 'vue'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import { Window } from '@tauri-apps/api/window'

const router = useRouter() as unknown as Router
const { currentLang } = useI18n()

// 获取当前窗口实例
const appWindow = new Window('main')

// 登录状态管理
const isLoggedIn = ref(!!localStorage.getItem('apiKey'))
const showLoginOverlay = computed(() => !isLoggedIn.value)

// 处理登录成功
function handleLoginSuccess() {
  isLoggedIn.value = true
}

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions = computed(() => [
  {
    label: messages[currentLang.value].menu.dashboard,
    key: 'dashboard',
    icon: renderIcon(HomeSharp)
  },
  {
    label: messages[currentLang.value].menu.history,
    key: 'history',
    icon: renderIcon(TimeSharp)
  },
  {
    label: messages[currentLang.value].menu.settings,
    key: 'settings',
    icon: renderIcon(SettingsSharp)
  }
])

function handleMenuClick(key: string) {
  router.push(`/${key}`)
}

const collapsed = ref(true)
const contentMarginLeft = computed(() => collapsed.value ? '64px' : '200px')

// 窗口控制函数
async function minimizeWindow() {
  await appWindow.minimize()
}

async function closeWindow() {
  await appWindow.hide()
}

</script>

<template>
  <n-layout has-sider style="height: 100vh">
    <!-- 可拖动区域 -->
    <div class="drag-region"></div>

    <!-- 登录遮罩 -->
    <login-overlay
      v-if="showLoginOverlay"
      @login-success="handleLoginSuccess"
    />

    <!-- 窗口控制按钮 -->
    <div class="window-controls">
      <n-button text @click="minimizeWindow" class="control-button">
        <template #icon>
          <n-icon :component="RemoveOutline" />
        </template>
      </n-button>
      <n-button text @click="closeWindow" class="control-button">
        <template #icon>
          <n-icon :component="Close" />
        </template>
      </n-button>
    </div>

    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="200"
      :collapsed="collapsed"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
      :native-scrollbar="false"
      position="absolute"
      style="height: 100vh; -webkit-app-region: drag"
    >
      <div class="logo">
        <h2 v-if="!collapsed" style="user-select: none;">Cursor Pool</h2>
        <h2 v-else style="user-select: none;">CP</h2>
      </div>
      <n-menu
        :options="menuOptions"
        :collapsed="collapsed"
        :collapsed-width="64"
        :collapsed-icon-size="22"
        :default-value="menuOptions[0].key"
        @update:value="handleMenuClick"
        style="-webkit-app-region: no-drag"
      />
      <div class="sider-footer" style="-webkit-app-region: no-drag">
        <theme-toggle style="-webkit-app-region: no-drag" />
      </div>
    </n-layout-sider>
    <n-layout 
      :native-scrollbar="false" 
      content-style="padding: 40px 24px 24px 24px; min-height: 100vh"
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

.drag-region {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 32px; /* 拖动区域高度 */
  -webkit-app-region: drag; /* 启用拖动 */
  z-index: 999;
}

.window-controls {
  position: fixed;
  top: 0;
  right: 0;
  z-index: 1000;
  display: flex;
  gap: 8px;
  padding: 8px;
}

/* 控制按钮样式 */
.control-button {
  color: var(--n-text-color) !important;
  transition: color 0.3s ease;
}

.control-button:hover {
  color: var(--n-text-color-hover) !important;
  background-color: var(--n-color-hover) !important;
}
</style>