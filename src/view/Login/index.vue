<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from '@/lib/i18n'
import { useTheme } from '@/lib/useTheme'
import { Sun, Moon, Globe, User, Lock, Loader2, Compass, Check } from '@lucide/vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Checkbox } from '@/components/ui/checkbox'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

const { t, locale, setLocale } = useI18n()
const { theme, toggleTheme } = useTheme()

const username = ref('')
const password = ref('')
const rememberMe = ref(true)
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
  if (!username.value || !password.value) return
  isLoading.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 1500))
    console.log('Login:', { username: username.value, password: password.value, remember: rememberMe.value })
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
  <div class="relative min-h-screen w-full flex items-center justify-center overflow-hidden transition-colors duration-500 bg-slate-50 text-slate-900 dark:bg-slate-950 dark:text-slate-50">

    <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-indigo-200/20 via-slate-50 to-slate-100 dark:from-indigo-950/40 dark:via-slate-950 dark:to-black pointer-events-none"></div>
    <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-blue-400/10 dark:bg-blue-500/10 rounded-full blur-[120px] animate-pulse pointer-events-none"></div>
    <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-purple-400/10 dark:bg-purple-500/10 rounded-full blur-[120px] animate-pulse duration-4000 pointer-events-none"></div>

    <div class="absolute top-6 right-6 flex items-center gap-2 z-50 animate-fade-in">
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="ghost" size="sm" class="gap-2 border border-slate-200 bg-white/50 dark:border-slate-800 dark:bg-slate-900/50 backdrop-blur-md hover:bg-slate-100 dark:hover:bg-slate-800/80 transition-colors">
            <Globe class="h-4 w-4 text-blue-500 dark:text-blue-400 animate-spin-slow" />
            <span class="text-xs font-medium">{{ currentLangLabel }}</span>
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" class="bg-white/90 dark:bg-slate-900/90 backdrop-blur-md border-slate-200 dark:border-slate-800">
          <DropdownMenuItem
              v-for="lang in languages"
              :key="lang.code"
              @click="switchLanguage(lang.code)"
              class="flex items-center justify-between cursor-pointer text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 focus:bg-slate-100 dark:focus:bg-slate-800"
          >
            <span class="text-xs font-medium">{{ lang.label }}</span>
            <Check v-if="locale === lang.code" class="h-4 w-4 text-blue-500 dark:text-blue-400" />
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <Button
          variant="ghost"
          size="icon"
          @click="toggleTheme"
          class="border border-slate-200 bg-white/50 dark:border-slate-800 dark:bg-slate-900/50 backdrop-blur-md hover:bg-slate-100 dark:hover:bg-slate-800/80 rounded-full transition-transform active:scale-95"
      >
        <Sun v-if="theme === 'dark'" class="h-4 w-4 text-amber-500 dark:text-amber-400 animate-bounce-subtle" />
        <Moon v-else class="h-4 w-4 text-indigo-600" />
      </Button>
    </div>

    <div class="relative w-full max-w-md p-1 group z-10 mx-4">
      <div class="absolute inset-0 bg-gradient-to-r from-blue-400 via-indigo-400 to-purple-500 dark:from-blue-500 dark:via-indigo-500 dark:to-purple-600 rounded-2xl blur-md opacity-15 group-hover:opacity-30 dark:group-hover:opacity-40 transition duration-1000 group-hover:duration-200 animate-tilt"></div>

      <Card class="relative border-slate-200/80 bg-white/70 text-slate-900 dark:border-slate-800/80 dark:bg-slate-900/60 backdrop-blur-xl shadow-2xl rounded-2xl overflow-hidden transition-all duration-300 hover:shadow-blue-500/5 dark:hover:shadow-blue-500/10">

        <CardHeader class="space-y-2 text-center pt-8 pb-4">
          <div class="flex justify-center mb-2">
            <div class="relative flex items-center justify-center w-16 h-16 rounded-full bg-white dark:bg-slate-950 border border-slate-200 dark:border-slate-800 group-hover:border-blue-400 dark:group-hover:border-blue-500/50 transition-colors duration-500 shadow-sm dark:shadow-inner">
              <Compass class="h-8 w-8 text-blue-500 dark:text-blue-400 animate-compass" />
              <div class="absolute inset-0 rounded-full border border-blue-400/20 dark:border-blue-500/20 animate-ping opacity-20"></div>
            </div>
          </div>
          <CardTitle class="text-3xl font-bold tracking-widest bg-gradient-to-r from-slate-900 to-slate-600 dark:from-white dark:via-slate-200 dark:to-slate-400 bg-clip-text text-transparent font-mono uppercase">
            {{ t('login.title') }}
          </CardTitle>
          <CardDescription class="text-slate-500 dark:text-slate-400 text-xs tracking-wider">
            {{ t('login.subtitle') }}
          </CardDescription>
        </CardHeader>

        <CardContent>
          <form @submit.prevent="handleLogin" class="space-y-5">

            <div class="space-y-2">
              <Label for="username" class="text-xs font-semibold text-slate-600 dark:text-slate-300 tracking-wider">
                {{ t('login.username') }}
              </Label>
              <div class="relative transition-all duration-300 focus-within:translate-x-1">
                <User class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-slate-400 dark:text-slate-500" />
                <Input
                    id="username"
                    v-model="username"
                    type="text"
                    :placeholder="t('login.username.placeholder')"
                    class="pl-10 bg-white/50 dark:bg-slate-950/50 border-slate-200 dark:border-slate-800 text-slate-900 dark:text-slate-100 placeholder:text-slate-400 dark:placeholder:text-slate-600 focus-visible:ring-1 focus-visible:ring-blue-500 focus-visible:border-blue-500"
                    required
                    style="padding-left: 2rem"
                />
              </div>
            </div>

            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <Label for="password" class="text-xs font-semibold text-slate-600 dark:text-slate-300 tracking-wider">
                  {{ t('login.password') }}
                </Label>
                <a href="#" class="text-xs text-blue-500 dark:text-blue-400 hover:underline transition-colors">
                  {{ t('login.forgot') }}
                </a>
              </div>
              <div class="relative transition-all duration-300 focus-within:translate-x-1">
                <Lock class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-slate-400 dark:text-slate-500" />
                <Input
                    id="password"
                    v-model="password"
                    type="password"
                    :placeholder="t('login.password.placeholder')"
                    class="pl-10 bg-white/50 dark:bg-slate-950/50 border-slate-200 dark:border-slate-800 text-slate-900 dark:text-slate-100 placeholder:text-slate-400 dark:placeholder:text-slate-600 focus-visible:ring-1 focus-visible:ring-blue-500 focus-visible:border-blue-500"
                    required
                    style="padding-left: 2rem"
                />
              </div>
            </div>

            <div class="flex items-center space-x-2 pt-1">
              <Checkbox
                  id="remember"
                  :checked="rememberMe"
                  @update:checked="(val) => rememberMe = !!val"
                  class="border-slate-300 dark:border-slate-700 data-[state=checked]:bg-blue-600 data-[state=checked]:border-blue-600"
                  style="padding:0"
              />
              <label for="remember" class="text-xs text-slate-500 dark:text-slate-400 cursor-pointer select-none hover:text-slate-700 dark:hover:text-slate-300 transition-colors">
                {{ t('login.remember') }}
              </label>
            </div>

            <Button
                type="submit"
                :disabled="isLoading"
                class="w-full mt-2 relative overflow-hidden bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-medium transition-all duration-300 active:scale-[0.98] shadow-lg shadow-blue-500/10 dark:shadow-blue-600/20 disabled:opacity-70"
            >
              <span class="absolute inset-0 w-full h-full bg-gradient-to-r from-transparent via-white/15 to-transparent -translate-x-full animate-shimmer"></span>

              <span class="flex items-center justify-center gap-2">
                <Loader2 v-if="isLoading" class="h-4 w-4 animate-spin" />
                <span>{{ isLoading ? t('login.loading') : t('login.submit') }}</span>
              </span>
            </Button>
          </form>
        </CardContent>

        <CardFooter class="justify-center border-t border-slate-100 dark:border-slate-800/50 bg-slate-50/50 dark:bg-slate-950/30 py-4 text-center">
          <p class="text-[10px] font-mono tracking-widest text-slate-400 dark:text-slate-500 uppercase">
            &copy; 2026 Polaris Project. All space reserved.
          </p>
        </CardFooter>
      </Card>
    </div>
  </div>
</template>

<style scoped>
/* 🧭 罗盘非线性拟真旋转 */
@keyframes compass-turn {
  0% { transform: rotate(0deg); }
  30% { transform: rotate(190deg); }
  60% { transform: rotate(170deg); }
  100% { transform: rotate(360deg); }
}
.animate-compass {
  animation: compass-turn 6s cubic-bezier(0.68, -0.6, 0.32, 1.6) infinite;
}

/* 🌐 地球旋转速度减缓，显得更优雅 */
.animate-spin-slow {
  animation: spin 12s linear infinite;
}

/* ☀️ 太阳微跃迁 */
@keyframes bounce-subtle {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}
.animate-bounce-subtle {
  animation: bounce-subtle 2s ease-in-out infinite;
}

/* ✨ 按钮冷光横扫动效 */
@keyframes shimmer {
  100% { transform: translateX(100%); }
}
.animate-shimmer {
  animation: shimmer 2.5s infinite;
}

/* 渐入登场动效 */
.animate-fade-in {
  animation: fadeIn 0.8s ease-out forwards;
}
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-5px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>