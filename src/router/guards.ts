import { Router } from 'vue-router'

export function setupRouterGuards(router: Router) {
  router.beforeEach((to, from, next) => {
    // 检查是否已登录
    const isLoggedIn = localStorage.getItem('token')
    
    if (to.name !== 'login' && !isLoggedIn) {
      next({ name: 'login' })
    } else if (to.name === 'login' && isLoggedIn) {
      next({ name: 'dashboard' })
    } else {
      next()
    }
  })
} 