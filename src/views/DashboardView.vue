<script setup lang="ts">
import { NCard, NSpace, NButton, NStatistic, NGrid, NGridItem } from 'naive-ui'
import { ref } from 'vue'

const deviceInfo = ref({
  machineCode: 'XXXX-XXXX-XXXX-XXXX',
  currentAccount: 'user@example.com',
  requestsToday: 50,
  remainingRequests: 150
})

const handleQuickChange = async () => {
  // TODO: 实现一键更换功能
}
</script>

<template>
  <n-space vertical :size="24">
    <n-grid :cols="2" :x-gap="24">
      <n-grid-item>
        <n-card title="设备信息">
          <n-space vertical>
            <n-statistic label="机器码">
              {{ deviceInfo.machineCode }}
            </n-statistic>
            <n-statistic label="当前账户">
              {{ deviceInfo.currentAccount }}
            </n-statistic>
          </n-space>
        </n-card>
      </n-grid-item>
      <n-grid-item>
        <n-card title="使用统计">
          <n-space vertical>
            <n-statistic label="今日请求次数">
              {{ deviceInfo.requestsToday }}
            </n-statistic>
            <n-statistic label="剩余请求次数">
              {{ deviceInfo.remainingRequests }}
            </n-statistic>
          </n-space>
        </n-card>
      </n-grid-item>
    </n-grid>
    
    <n-card title="快捷操作">
      <n-space>
        <n-button type="primary" @click="handleQuickChange">
          一键更换(账户+机器码)
        </n-button>
      </n-space>
    </n-card>
  </n-space>
</template> 