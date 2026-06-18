import { ref, onMounted } from 'vue'

type Theme = 'light' | 'dark'

const currentTheme = ref<Theme>(
  (localStorage.getItem('theme') as Theme) || 'light'
)

export function useTheme() {
  function applyTheme(theme: Theme) {
    const root = document.documentElement
    if (theme === 'dark') {
      root.classList.add('dark')
    } else {
      root.classList.remove('dark')
    }
  }

  function setTheme(theme: Theme) {
    currentTheme.value = theme
    localStorage.setItem('theme', theme)
    applyTheme(theme)
  }

  function toggleTheme() {
    setTheme(currentTheme.value === 'light' ? 'dark' : 'light')
  }

  onMounted(() => {
    applyTheme(currentTheme.value)
  })

  return {
    theme: currentTheme,
    setTheme,
    toggleTheme,
  }
}
