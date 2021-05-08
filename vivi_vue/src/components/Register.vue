<template>
    <body class="background">
        <el-form :model="infoForm" :rules="rules" class="register-form" label-position="left" label-width="80px" ref="infoForm">
            <h3 class="title">注 册</h3>
            <el-form-item label="用户名" prop="username">
                <el-input placeholder="请输入2-10位用户名" class="input" v-model="infoForm.username"></el-input>
            </el-form-item>
            <el-form-item label="密码" prop="password">
                <el-input placeholder="请输入密码" v-model="infoForm.password"></el-input>
            </el-form-item>
            <el-form-item label="确认密码" prop="check">
                <el-input placeholder="请再次输入密码" v-model="infoForm.check"></el-input>
            </el-form-item>
            <el-form-item label="邮箱">
                <el-input placeholder="请输入邮箱" v-model="infoForm.mail"></el-input>
            </el-form-item>
            <el-button type="primary" v-on:click="register('infoForm')" class="register_button">注册</el-button>
            <el-link :href="this.loginUrl" style="float:right;" :underline="false">已有账号？登录</el-link>
        </el-form>
    </body>
</template>
<script>
export default {
    name:'Register',
    data(){
         /*表单检验 */
            var passCheck=(rule,value,callback)=>{
                if(value==''){
                    callback(new Error('请确认密码'));
                }
                else{
                    if(value!=this.infoForm.password){
                        callback(new Error('两次密码输入不相同'));
                    }
                    else{
                        callback();
                    }
                }
            };
        return{
            loginUrl:'/login',
            /*表单信息*/
            infoForm:{
                username:'',
                password:'',
                check:'',
                mail:'',
            },
            /*表单规则 */
            rules:{
                username:[
                    {required:true,message:'请输入用户名',trigger:'blur'},
                    {min:2,max:10,message:'长度在2-10个字符',trigger:'blur'},
                ],
                password:[
                    {required:true,message:'密码不能为空',trigger:'blur'},
                    {min:8,max:16,message:'密码长度不能超过16个字符',trigger:'change'},
                ],
                check:[
                    {validator:passCheck,trigger:'blur'},
                ]
            }
        }
    },
    methods:{
        register(formName){
            /*validate这是属于element-ui的方法 */
            this.$refs[formName].validate((valid)=>{
                //表单验证通过
                if(valid){
                    alert('submit');
                }
                else{
                    alert('fail');
                    return false;
                }
            });
        }
    }
    
}
</script>
<style scoped>
.background{
    width: 100%;
    height: 100%;
    background:url("../assets/img/rbg.jpg") no-repeat;
    background-size: cover;
    position: fixed;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    background-color: black;
    background-position:20%;
}
.register-form{
    border-radius: 15px;
    width: 350px;
    border: 1px solid #eaeaea;
    padding: 15px 35px 15px 35px;
    box-shadow: 0 0 25px #C0C4CC;
    background-color: rgba(255, 255, 255, 0.1);
}
/*因为是内部类，所以需要穿透 */
.el-input /deep/ .el-input__inner:focus{
    border-color:#C0C4CC;
}

 .register_button{
    width: 100%;
    opacity: 100%;
    background: #818283;
    border: none;
    margin-bottom: 20px;
  }
  .title{
    color:#818283;
  }
</style>