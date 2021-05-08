import Vue from 'vue'
import Router from 'vue-router'
import AppIndex from '@/components/home/AppIndex'
import FollowIndex from '@/components/home/FollowIndex.vue'
import Login from '@/components/Login'
import Home from '@/components/Home.vue'
import UserCenter from '@/components/home/UserCenter'
import Register from '@/components/Register'
import Publish from '@/components/home/Publish'
import Article from '@/components/home/Article'

Vue.use(Router)

export default new Router({
  mode:'history',
  routes: [
    {
      path:'/home',
      name:'Home',
      component:Home,
      //home不需要被访问
      redirect:'/index',
      children:[
        {
          path:'/index',
          name:'AppIndex',
          component:AppIndex,
        },
        {
          path:'/follow',
          name:'FollowIndex',
          component:FollowIndex,
          meta:{requireAuth:true}
        },
        {
          path:'/usercenter/:id',
          name:'UserCenter',
          component:UserCenter,
          meta:{requireAuth:true}
        },
        {
          path:'/publish',
          name:'Publish',
          component:Publish,
          meta:{requireAuth:true}
        },
        {
          path:'/article/:id',
          name:'Article',
          component:Article
        }
      ]
    },
    {
      path: '/login',
      name: 'Login',
      component: Login,
      meta:{requireAuth: true}
    },
    {
      path:'/register',
      name:'Register',
      component:Register
    }
  ]
})
