<script setup lang="ts">
import { NCard, NSpace, NButton, NProgress, NNumberAnimation, NGrid, NGridItem } from 'naive-ui'
import { ref } from 'vue'
import type { NumberAnimationInst } from 'naive-ui'
import { useI18n } from '../locales'
import { messages } from '../locales/messages'

const deviceInfo = ref({
  machineCode: 'XXXX-XXXX-XXXX-XXXX',
  currentAccount: 'user@example.com',
  expiryDate: '2024-12-31',
  usageStats: {
    advanced: {
      used: 15,
      max: 150
    },
    normal: {
      used: 50,
      max: Infinity
    }
  }
})

const advancedAnimationRef = ref<NumberAnimationInst | null>(null)
const normalAnimationRef = ref<NumberAnimationInst | null>(null)

const { currentLang } = useI18n()

const handleQuickChange = async () => {
  // TODO: 实现一键更换功能
}

const handleAccountChange = async () => {
  // TODO: 实现账户更换功能
}

const handleMachineCodeChange = async () => {
  // TODO: 实现机器码更换功能
}
</script>

<template>
  <n-space vertical :size="24">
    <n-grid :cols="2" :x-gap="24">
      <n-grid-item>
        <n-card :title="messages[currentLang].dashboard.deviceInfo">
          <n-space vertical>
            <div>{{ messages[currentLang].dashboard.machineCode }}：{{ deviceInfo.machineCode }}</div>
            <div>{{ messages[currentLang].dashboard.currentAccount }}：{{ deviceInfo.currentAccount }}</div>
            <div>{{ messages[currentLang].dashboard.expiryDate }}：{{ deviceInfo.expiryDate }}</div>
          </n-space>
        </n-card>
      </n-grid-item>
      <n-grid-item>
        <n-card :title="messages[currentLang].dashboard.usageStats">
          <n-space vertical>
            <div>
              <div style="display: flex; justify-content: space-between; margin-bottom: 6px">
                <span>{{ messages[currentLang].dashboard.advancedModel }}</span>
                <span>
                  <n-number-animation 
                    ref="advancedAnimationRef"
                    :from="0" 
                    :to="deviceInfo.usageStats.advanced.used"
                  />
                  /{{ deviceInfo.usageStats.advanced.max }}
                </span>
              </div>
              <n-progress
                type="line"
                :percentage="(deviceInfo.usageStats.advanced.used / deviceInfo.usageStats.advanced.max) * 100"
                :show-indicator="false"
              />
            </div>
            <div>
              <div style="display: flex; justify-content: space-between; margin-bottom: 6px">
                <span>{{ messages[currentLang].dashboard.normalModel }}</span>
                <span>
                  <n-number-animation 
                    ref="normalAnimationRef"
                    :from="0" 
                    :to="deviceInfo.usageStats.normal.used"
                  />
                </span>
              </div>
              <n-progress
                type="line"
                :percentage="100"
                :show-indicator="false"
              />
            </div>
          </n-space>
        </n-card>
      </n-grid-item>
    </n-grid>
    
    <n-card :title="messages[currentLang].dashboard.quickActions">
      <n-space>
        <n-button type="primary" @click="handleQuickChange">
          {{ messages[currentLang].dashboard.quickChange }}
        </n-button>
        <n-button type="primary" @click="handleAccountChange">
          {{ messages[currentLang].dashboard.changeAccount }}
        </n-button>
        <n-button type="primary" @click="handleMachineCodeChange">
          {{ messages[currentLang].dashboard.changeMachineCode }}
        </n-button>
      </n-space>
    </n-card>
  </n-space>
</template> 