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

// 从 localStorage 读取语言设置
const savedLang = localStorage.getItem('language') as Language
export const currentLang = ref<Language>(savedLang || 'zh-CN')

export function useI18n() {
  const setLanguage = (lang: Language) => {
    currentLang.value = lang
    localStorage.setItem('language', lang)
  }

  return {
    currentLang,
    setLanguage,
    i18n: computed(() => messages[currentLang.value])
  }
} 