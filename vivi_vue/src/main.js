import Vue from 'vue'
import App from './App'
import router from './router'
import ElementUI, { TabPane } from 'element-ui'
import 'element-ui/lib/theme-chalk/index.css'
import store from './store'
import '#/icon/iconfont.css'

var axios = require('axios')
const urls = [
  '/user/update/info',
  '/user/update/avatar',
  '/user/update/pwd',
  '/article/publish',
  '/article/delete',
  '/comment/publish',
]

axios.defaults.baseURL = '/api'
//axios.defaults.baseURL = 'http://localhost:8080/api'
Vue.prototype.$axios = axios
Vue.config.productionTip = false

//axios.defaults.headers.common['token'] = store.state.token;

Vue.use(ElementUI)
//路由守卫
router.beforeEach((to, from, next) => {
  if (to.meta.requireAuth) {
    if (store.state.user.username) {
      if (to.path == "/login"){
        next({
          path: 'home'
        })
      }
      else{
        next()
      }
    } else {
      if (to.path == "/login"){
        next()
      }
      else{
        next({
          path: 'login',
          query: {redirect: to.fullPath}
        })
      }
    }
  } else {
    next()
  }
})

/*请求拦截 */
axios.interceptors.request.use((request) => {
  let url = request.url.replace(axios.default.baseURL,"");
  //需要添加token
  if (urls.includes(url)){
    let token = store.state.token;
    if (token){
      request.headers.token = token;
      console.log('set token')
    }else{
      this.$message({
        message: '请先登录'
      });
      console.log('reject')
      return;
    }
  }
  return request;
  //再发送给后台
}, function (error) {
  // Do something with request error
  return Promise.reject(error);
});

/*回复拦截 */
//response拦截器
axios.interceptors.response.use(res  =>  { 
  return res;
},  error  =>  {
  return Promise.reject(error.response.data) // 返回错误信息
})
/* eslint-disable no-new */
new Vue({
  el: '#app',
  render: h => h(App),
  router,
  store,
  components: { App },
  template: '<App/>'
})

