<script setup lang="ts">
import { NLayout, NLayoutSider, NLayoutContent, NMenu } from 'naive-ui'
import { ref } from 'vue'
import { RouterView, useRouter } from 'vue-router'
import { 
  DashboardOutlined, 
  SettingsOutlined,
  UserOutlined,
  HistoryOutlined 
} from '@vicons/antd'
import ThemeToggle from '../components/ThemeToggle.vue'

const router = useRouter()
const menuOptions = [
  {
    label: '概览',
    key: 'dashboard',
    icon: DashboardOutlined,
    path: '/'
  },
  {
    label: '账户管理',
    key: 'account',
    icon: UserOutlined,
    path: '/account'
  },
  {
    label: '操作记录',
    key: 'history',
    icon: HistoryOutlined,
    path: '/history'
  },
  {
    label: '设置',
    key: 'settings',
    icon: SettingsOutlined,
    path: '/settings'
  }
]

const handleMenuClick = (key: string) => {
  const menuItem = menuOptions.find(item => item.key === key)
  if (menuItem) {
    router.push(menuItem.path)
  }
}
</script>

<template>
  <n-layout has-sider>
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="240"
      show-trigger
      class="sider"
    >
      <div class="logo">
        <h2>Cursor Pool</h2>
      </div>
      <n-menu
        :options="menuOptions"
        :default-value="menuOptions[0].key"
        @update:value="handleMenuClick"
      />
      <div class="sider-footer">
        <theme-toggle />
      </div>
    </n-layout-sider>
    <n-layout-content content-style="padding: 24px;">
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<style scoped>
.sider {
  height: 100vh;
  position: relative;
}

.logo {
  padding: 16px;
  text-align: center;
}

.sider-footer {
  position: absolute;
  bottom: 16px;
  left: 0;
  right: 0;
  display: flex;
  justify-content: center;
}
</style> 