<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from '@/lib/i18n'
import { useTheme } from '@/lib/useTheme'
import { Sun, Moon, Globe, User, Lock, Loader2 } from '@lucide/vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'

const { t, locale, setLocale } = useI18n()
const { theme, toggleTheme } = useTheme()

const username = ref('')
const password = ref('')
const rememberMe = ref(false)
const isLoading = ref(false)

const languages = [
  { code: 'zh', label: '中文' },
  { code: 'en', label: 'English' },
  { code: 'ja', label: '日本語' },
]

const currentLangLabel = computed(() => {
  return languages.find(l => l.code === locale.value)?.label || '中文'
})

async function handleLogin() {
  if (!username.value || !password.value) {
    return
  }

  isLoading.value = true
  
  try {
    // 模拟登录请求
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    // 这里可以调用实际的登录 API
    console.log('Login:', { username: username.value, password: password.value, remember: rememberMe.value })
    
    // 登录成功后跳转
    // router.push('/dashboard')
  } catch (error) {
    console.error('Login failed:', error)
  } finally {
    isLoading.value = false
  }
}

function switchLanguage(code: string) {
  setLocale(code as 'zh' | 'en' | 'ja')
}
</script>

<template>
  <div class="min-h-screen w-full flex items-center justify-center p-4 bg-gradient-to-br from-background via-muted/30 to-background transition-colors duration-300">
    <!-- 顶部工具栏 -->
    <div class="fixed top-4 right-4 flex items-center gap-2 z-50">
      <!-- 主题切换 -->
      <Button
        variant="ghost"
        size="icon"
        @click="toggleTheme"
        class="rounded-full"
      >
        <Sun v-if="theme === 'light'" class="h-5 w-5" />
        <Moon v-else class="h-5 w-5" />
      </Button>

      <!-- 语言切换 -->
      <div class="relative group">
        <Button
          variant="ghost"
          size="sm"
          class="rounded-full gap-2"
        >
          <Globe class="h-4 w-4" />
          <span class="hidden sm:inline">{{ currentLangLabel }}</span>
        </Button>
        
        <div class="absolute right-0 top-full mt-2 w-32 bg-card border border-border rounded-lg shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200">
          <div class="p-1">
            <button
              v-for="lang in languages"
              :key="lang.code"
              @click="switchLanguage(lang.code)"
              class="w-full text-left px-3 py-2 text-sm rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
              :class="{ 'bg-accent': locale === lang.code }"
            >
              {{ lang.label }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 登录卡片 -->
    <Card class="w-full max-w-md shadow-2xl border-border/50 backdrop-blur-sm bg-card/95">
      <CardHeader class="space-y-3 pb-6">
        <div class="flex justify-center">
          <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-primary to-primary/60 flex items-center justify-center shadow-lg">
            <svg class="h-8 w-8 text-primary-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
          </div>
        </div>
        
        <CardTitle class="text-2xl font-bold text-center">
          {{ t('login.title') }}
        </CardTitle>
        <CardDescription class="text-center">
          {{ t('login.subtitle') }}
        </CardDescription>
      </CardHeader>

      <CardContent class="space-y-5">
        <form @submit.prevent="handleLogin" class="space-y-5">
          <!-- 用户名 -->
          <div class="space-y-2">
            <Label for="username" class="text-sm font-medium">
              {{ t('login.username') }}
            </Label>
            <div class="relative">
              <User class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input
                id="username"
                v-model="username"
                type="text"
                :placeholder="t('login.username.placeholder')"
                class="pl-10 h-11"
                :disabled="isLoading"
                required
              />
            </div>
          </div>

          <!-- 密码 -->
          <div class="space-y-2">
            <Label for="password" class="text-sm font-medium">
              {{ t('login.password') }}
            </Label>
            <div class="relative">
              <Lock class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input
                id="password"
                v-model="password"
                type="password"
                :placeholder="t('login.password.placeholder')"
                class="pl-10 h-11"
                :disabled="isLoading"
                required
              />
            </div>
          </div>

          <!-- 记住我 & 忘记密码 -->
          <div class="flex items-center justify-between">
            <label class="flex items-center gap-2 cursor-pointer">
              <input
                v-model="rememberMe"
                type="checkbox"
                class="h-4 w-4 rounded border-border accent-primary"
                :disabled="isLoading"
              />
              <span class="text-sm text-muted-foreground">
                {{ t('login.remember') }}
              </span>
            </label>
            <a
              href="#"
              class="text-sm text-primary hover:underline underline-offset-4"
            >
              {{ t('login.forgot') }}
            </a>
          </div>

          <!-- 登录按钮 -->
          <Button
            type="submit"
            class="w-full h-11 text-base font-medium"
            :disabled="isLoading || !username || !password"
          >
            <Loader2 v-if="isLoading" class="h-4 w-4 animate-spin" />
            <span v-else>{{ t('login.submit') }}</span>
          </Button>
        </form>
      </CardContent>

      <CardFooter class="flex-col space-y-3 pt-4">
        <div class="text-center text-sm text-muted-foreground">
          {{ t('login.no.account') }}
          <a
            href="#"
            class="font-medium text-primary hover:underline underline-offset-4"
          >
            {{ t('login.sign.up') }}
          </a>
        </div>
      </CardFooter>
    </Card>
  </div>
</template>

<style scoped>
/* 响应式调整 */
@media (max-width: 640px) {
  .max-w-md {
    max-width: 100%;
  }
}

/* 卡片进入动画 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.card {
  animation: fadeInUp 0.5s ease-out;
}
</style>
