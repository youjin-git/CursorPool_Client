import type { Router, RouteLocationNormalized, NavigationGuardNext } from 'vue-router'

export function setupRouterGuards(router: Router) {
  router.beforeEach((
    to: RouteLocationNormalized,
    _from: RouteLocationNormalized,
    next: NavigationGuardNext
  ) => {
    const api_key = localStorage.getItem('api_key')
    
    if (to.name !== 'login' && !api_key) {
      next({ name: 'login' })
    } else if (to.name === 'login' && api_key) {
      next({ name: 'dashboard' })
    } else {
      next()
    }
  })
}