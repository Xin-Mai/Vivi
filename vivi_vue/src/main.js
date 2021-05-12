import Vue from 'vue'
import App from './App'
import router from './router'
import ElementUI, { TabPane } from 'element-ui'
import 'element-ui/lib/theme-chalk/index.css'
import store from './store'
import '#/icon/iconfont.css'

var axios = require('axios')
axios.defaults.baseURL = 'http://localhost:8080/api'
Vue.prototype.$axios = axios
Vue.config.productionTip = false

//axios.defaults.headers.common['token'] = store.state.token;

Vue.use(ElementUI)

router.beforeEach((to, from, next) => {
  if (to.meta.requireAuth) {
    console.log(to.path == "/login");
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

/* eslint-disable no-new */
new Vue({
  el: '#app',
  render: h => h(App),
  router,
  store,
  components: { App },
  template: '<App/>'
})

