import type { Router } from 'vue-router'

export function setupRouterGuards(router: Router) {
  (router as any).beforeEach((to: any, _from: any, next: any) => {
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