<template>
<body class="background">
  <el-form label-positon="left" label-width="0px" class="login_container">
    <h3 class="login_title">登录</h3>
    <el-form-item>
      <el-input v-model="loginForm.username" placeholder="用户名"></el-input>
    </el-form-item>
    <el-form-item>
      <el-input v-model="loginForm.password" placeholder="密码" show-password></el-input>
    </el-form-item>
    <el-form-item>
      <el-button type="primary" v-on:click="login" class="login_button">登录</el-button>
    </el-form-item>
  </el-form>
</body>
</template>

<script>
  export default {
  name: 'Login',
  data () {
    return {
     loginForm:{
        username:'',
        password:''
     },
     responseResult:[]
    }
  },
  methods:{
    login(){
      var _this=this
      this.$axios
        .post('/login',{
          username:this.loginForm.username,
          password:this.loginForm.password
        })
        .then(successResponse=>{
          if(successResponse.data.code==200){
            _this.$store.commit('login',this.loginForm)
            var path=_this.$route.query.redirect
            this.$router.replace({path: path === '/' || path === undefined ? '/index' : path})
          }
        })
        .catch(failResponse=>{
        })
    }
  }
}
</script>

<style scoped>
  .background{
    background:url("../assets/bg2.jpg") no-repeat;
    background-position: center;
    height: 100%;
    width: 100%;
    background-size: contain;
    background-color:black;
    position: fixed;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .login_container{
    border-radius: 15px;
    width: 350px;
    border: 1px solid #eaeaea;
    padding: 15px 35px 15px 35px;
    box-shadow: 0 0 25px #cac6c6;
  }
  .login_title{
    color: #e6ebf0;
  }
  .login_button{
    width: 100%;
    background: #818283;
    border: none;
  }
</style>
