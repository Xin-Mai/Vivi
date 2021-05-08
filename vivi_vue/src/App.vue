<template>
  <div id="app">
    <router-view id="app-router-view" />
  </div>
</template>

<script>
export default {
  name: 'App'
}
</script>

<style>
html,body{
  width: 100%;
  height: 100%;
}
#app {
  min-height: 100%;
  position: relative;
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
#app-router-view{
  min-height: 100%;
  position: absolute;
  bottom:0;
  top:0;
  left:0;
  right: 0;
  overflow: hidden;
}
</style>
