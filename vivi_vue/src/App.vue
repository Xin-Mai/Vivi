<template>
  <div id="app">
    <router-view/>
  </div>
</template>

<script>
export default {
  name: 'App'
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
/*设置vue的menu的样式 */
.el-menu-item:hover { 
  background-color: transparent !important;
}
.el-menu-item.is-active{
  background-color: transparent !important;
  border-width: 0 0 2px 0;
  border-style:solid;
}
/*清除浏览器的默认代理属性*/
body{
  margin: 0px;
}
</style>
