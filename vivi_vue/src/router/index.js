import Vue from 'vue'
import Router from 'vue-router'
import AppIndex from '@/components/home/AppIndex'
import Login from '@/components/Login'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path:'/index',
      name:'AppIndex',
      component:AppIndex
    },
    {
      path: '/login',
      name: 'Login',
      component: Login
    }
  ]
})
