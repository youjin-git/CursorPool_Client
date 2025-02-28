<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { NModal, NSpace, NButton, NScrollbar } from 'naive-ui'
import { getDisclaimer } from '@/api'

const props = defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  (e: 'update:show', value: boolean): void
}>()

const content = ref('')
const countdown = ref(3)
const loading = ref(true)
const canConfirm = ref(false)

onMounted(async () => {
  try {
    const { content: disclaimerContent } = await getDisclaimer()
    content.value = disclaimerContent
    const timer = setInterval(() => {
      countdown.value--
      if (countdown.value <= 0) {
        canConfirm.value = true
        clearInterval(timer)
      }
    }, 1000)
  } catch (error) {
    console.error('获取免责声明失败:', error)
  } finally {
    loading.value = false
  }
})

const handleConfirm = () => {
  localStorage.setItem('disclaimer_accepted', 'true')
  emit('update:show', false)
  setTimeout(() => {
    window.location.reload()
  }, 100)
}
</script>

<template>
  <n-modal :show="show" preset="card" style="width: 600px; max-width: 90vw;" title="免责声明" :closable="false"
    :mask-closable="false">
    <n-scrollbar style="max-height: 60vh">
      <div style="white-space: pre-line; padding: 16px 0;">
        {{ content }}
      </div>
    </n-scrollbar>
    <template #footer>
      <n-space justify="end">
        <n-button type="primary" :disabled="!canConfirm" @click="handleConfirm">
          {{ canConfirm ? '我已阅读并同意' : `请等待 ${countdown} 秒` }}
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>