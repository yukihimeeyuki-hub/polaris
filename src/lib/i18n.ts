import { ref, computed } from 'vue'

type Locale = 'zh' | 'en' | 'ja'

const messages: Record<Locale, Record<string, string>> = {
  zh: {
    'login.title': '欢迎回来',
    'login.subtitle': '请登录您的账户',
    'login.username': '用户名',
    'login.username.placeholder': '请输入用户名',
    'login.password': '密码',
    'login.password.placeholder': '请输入密码',
    'login.remember': '记住我',
    'login.forgot': '忘记密码？',
    'login.submit': '登 录',
    'login.loading': '登录中...',
    'login.no.account': '还没有账户？',
    'login.sign.up': '立即注册',
    'login.error.required': '请输入{field}',
    'theme.light': '浅色模式',
    'theme.dark': '深色模式',
    'lang.label': '语言',
  },
  en: {
    'login.title': 'Welcome Back',
    'login.subtitle': 'Please sign in to your account',
    'login.username': 'Username',
    'login.username.placeholder': 'Enter your username',
    'login.password': 'Password',
    'login.password.placeholder': 'Enter your password',
    'login.remember': 'Remember me',
    'login.forgot': 'Forgot password?',
    'login.submit': 'Sign In',
    'login.loading': 'Signing in...',
    'login.no.account': "Don't have an account?",
    'login.sign.up': 'Sign up',
    'login.error.required': 'Please enter {field}',
    'theme.light': 'Light Mode',
    'theme.dark': 'Dark Mode',
    'lang.label': 'Language',
  },
  ja: {
    'login.title': 'おかえりなさい',
    'login.subtitle': 'アカウントにサインインしてください',
    'login.username': 'ユーザー名',
    'login.username.placeholder': 'ユーザー名を入力',
    'login.password': 'パスワード',
    'login.password.placeholder': 'パスワードを入力',
    'login.remember': 'ログイン状態を保持',
    'login.forgot': 'パスワードをお忘れですか？',
    'login.submit': 'サインイン',
    'login.loading': 'サインイン中...',
    'login.no.account': 'アカウントをお持ちでないですか？',
    'login.sign.up': '新規登録',
    'login.error.required': '{field}を入力してください',
    'theme.light': 'ライトモード',
    'theme.dark': 'ダークモード',
    'lang.label': '言語',
  },
}

const currentLocale = ref<Locale>(
  (localStorage.getItem('locale') as Locale) || 'zh'
)

export function useI18n() {
  const locale = computed(() => currentLocale.value)

  function t(key: string, params?: Record<string, string>): string {
    let text = messages[currentLocale.value]?.[key] ?? key
    if (params) {
      Object.entries(params).forEach(([k, v]) => {
        text = text.replace(`{${k}}`, v)
      })
    }
    return text
  }

  function setLocale(lang: Locale) {
    currentLocale.value = lang
    localStorage.setItem('locale', lang)
  }

  return { locale, t, setLocale }
}
