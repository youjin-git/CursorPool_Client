<script setup lang="ts">
import { NLayout, NLayoutSider, NMenu, NIcon, NButton } from 'naive-ui'
import { ref, computed, onMounted, onUnmounted } from 'vue'
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
import { Component, h } from 'vue'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'
import { Window } from '@tauri-apps/api/window'
import { TrayIcon } from '@tauri-apps/api/tray'
import { Menu, MenuItem } from '@tauri-apps/api/menu'
import { Image } from '@tauri-apps/api/image'

const router = useRouter() as unknown as Router
const { currentLang } = useI18n()

// 获取当前窗口实例
const appWindow = new Window('main')

// 创建系统托盘
let tray: TrayIcon | null = null

async function setupTray() {
  try {
    // 清理旧的托盘图标
    if (tray) {
      await tray.close()
      tray = null
    }

    const items = [
      await MenuItem.new({
        text: '一键换号',
        action: () => console.log('一键换号')
      }),
      await MenuItem.new({
        text: '换号',
        action: () => console.log('换号')
      }),
      await MenuItem.new({
        text: '换机器码',
        action: () => console.log('换机器码')
      }),
      await MenuItem.new({
        text: '打开界面',
        action: () => {
          appWindow.show()
          appWindow.unminimize()
        }
      }),
      await MenuItem.new({
        text: '退出',
        action: () => appWindow.close()
      })
    ]

    // 创建菜单
    const trayMenu = await Menu.new({
      items: items
    })

    // 加载图标数据
    const iconData = await Image.fromPath('icons/32x32.png')

    // 创建系统托盘图标
    tray = await TrayIcon.new({
      icon: iconData,
      tooltip: 'Cursor Pool',
      menu: trayMenu,
      showMenuOnLeftClick: false
    })

    // 左键点击事件
    appWindow.listen('tray-click', () => {
      appWindow.show()
      appWindow.unminimize()
    })

    // 右键点击事件
    appWindow.listen('tray-right-click', () => {
      trayMenu.popup()
    })
  } catch (error) {
    console.error('设置系统托盘时出错:', error)
  }
}

// 使用生命周期钩子来管理托盘图标
onMounted(() => {
  setupTray()
})

onUnmounted(async () => {
  if (tray) {
    await tray.close()
    tray = null
  }
})

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

// 窗口控制函数
const minimizeWindow = async () => {
  await appWindow.minimize()
  await appWindow.hide() // 隐藏窗口
}

const closeWindow = () => appWindow.close()
</script>

<template>
  <n-layout has-sider style="height: 100vh">
    <!-- 可拖动区域 -->
    <div class="drag-region"></div>

    <!-- 自定义窗口控制按钮 -->
    <div class="window-controls">
      <n-button text @click="minimizeWindow">
        <n-icon :component="RemoveOutline" />
      </n-button>
      <n-button text @click="closeWindow">
        <n-icon :component="Close" />
      </n-button>
    </div>

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
      style="height: 100vh; -webkit-app-region: drag"
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
        style="-webkit-app-region: no-drag"
      />
      <div class="sider-footer" style="-webkit-app-region: no-drag">
        <theme-toggle style="-webkit-app-region: no-drag" />
      </div>
    </n-layout-sider>
    <n-layout 
      :native-scrollbar="false" 
      content-style="padding: 24px; min-height: 100vh"
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
  background-color: rgba(255, 255, 255, 0.8); /* 可选：背景色 */
}
</style>