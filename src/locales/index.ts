import { ref, computed } from 'vue'
import type { NLocale, NDateLocale } from 'naive-ui'
import { 
  zhCN, 
  dateZhCN,
  enUS,
  dateEnUS,
  jaJP,
  dateJaJP,
  frFR,
  dateFrFR,
  deDE,
  dateDeDE,
  koKR,
  dateKoKR,
  ruRU,
  dateRuRU,
  esAR,
  dateEsAR
} from 'naive-ui'
import { messages } from './messages'
import { getUserData, setUserData } from '@/api'

export type Language = 'zh-CN' | 'en-US' | 'ja-JP' | 'fr-FR' | 'de-DE' | 'ko-KR' | 'ru-RU' | 'es-AR'

interface LocaleConfig {
  name: string
  locale: NLocale
  dateLocale: NDateLocale
}

export const locales: Record<Language, LocaleConfig> = {
  'zh-CN': {
    name: '简体中文',
    locale: zhCN,
    dateLocale: dateZhCN
  },
  'en-US': {
    name: 'English',
    locale: enUS,
    dateLocale: dateEnUS
  },
  'ja-JP': {
    name: '日本語',
    locale: jaJP,
    dateLocale: dateJaJP
  },
  'fr-FR': {
    name: 'Français',
    locale: frFR,
    dateLocale: dateFrFR
  },
  'de-DE': {
    name: 'Deutsch',
    locale: deDE,
    dateLocale: dateDeDE
  },
  'ko-KR': {
    name: '한국어',
    locale: koKR,
    dateLocale: dateKoKR
  },
  'ru-RU': {
    name: 'Русский',
    locale: ruRU,
    dateLocale: dateRuRU
  },
  'es-AR': {
    name: 'Español',
    locale: esAR,
    dateLocale: dateEsAR
  }
}

// 默认使用中文
export const currentLang = ref<Language>('zh-CN')

// 初始化语言设置
export async function initLanguage() {
  try {
    // 从后端获取语言设置
    const lang = await getUserData('user.info.lang') as Language | null
    
    // 如果后端存在语言设置且为受支持的语言则使用该设置
    if (lang && Object.keys(locales).includes(lang)) {
      currentLang.value = lang
    } else {
      // 如果不存在或不受支持，则使用中文并保存到后端
      await setUserData('user.info.lang', 'zh-CN')
    }
  } catch (error) {
    console.error('初始化语言设置失败:', error)
  }
}

export function useI18n() {
  const setLanguage = async (lang: Language) => {
    // 更新当前语言状态
    currentLang.value = lang
    
    // 保存到后端数据库
    try {
      await setUserData('user.info.lang', lang)
    } catch (err) {
      console.error('同步语言设置失败:', err)
    }
  }

  return {
    currentLang,
    setLanguage,
    i18n: computed(() => messages[currentLang.value])
  }
} 