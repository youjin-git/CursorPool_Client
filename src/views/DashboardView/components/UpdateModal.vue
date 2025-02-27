<script setup lang="ts">
import { NModal, NSpace, NButton, NDivider } from 'naive-ui'
import { useI18n } from '../../../locales'
import { version as LOCAL_VERSION } from '../../../../package.json'
import type { VersionInfo } from '@/api/types'

defineProps<{
  show: boolean
  versionInfo: VersionInfo | null
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
  (e: 'download'): void
  (e: 'later'): void
}>()

const { i18n } = useI18n()

const updateShow = (value: boolean) => {
  emit('update:show', value)
}

const handleDownload = () => {
  emit('download')
}

const handleLater = () => {
  emit('later')
}
</script>

<template>
  <n-modal
    :show="show"
    :mask-closable="!versionInfo?.forceUpdate"
    :closable="!versionInfo?.forceUpdate"
    preset="card"
    style="width: 500px"
    :title="i18n.dashboard.newVersionAvailable"
    @update:show="updateShow"
  >
    <n-space vertical>
      <div>{{ i18n.dashboard.currentVersion }}: {{ LOCAL_VERSION }}</div>
      <div>{{ i18n.dashboard.newVersion }}: {{ versionInfo?.version }}</div>
      <n-divider />
      <div style="white-space: pre-line">{{ versionInfo?.changeLog }}</div>
      <n-space justify="end">
        <n-button
          v-if="!versionInfo?.forceUpdate"
          @click="handleLater"
        >
          {{ i18n.dashboard.later }}
        </n-button>
        <n-button
          type="primary"
          @click="handleDownload"
        >
          {{ i18n.dashboard.downloadNow }}
        </n-button>
      </n-space>
    </n-space>
  </n-modal>
</template> 