<template>
    <el-form :model="infoForm" :rules="rules" :class="formClass" label-position="left" label-width="80px" ref="infoForm">
            <h3 v-show="type=='reg'" class="title">{{title}}</h3>
            <el-form-item label="用户名" prop="username">
                <el-input placeholder="请输入2-10位用户名" class="input"
                :disabled="type=='edit'"
                v-model="infoForm.username"></el-input>
            </el-form-item>
            <el-form-item  v-if="showEdit" label="个人介绍">
                <el-input placeholder="用30个字介绍你自己吧" class="input" v-model="editForm.intro"></el-input>
            </el-form-item>
            <el-form-item v-if="showEdit" label="修改密码">
                <el-button type="text" style="float:right" icon="el-icon-edit" @click="showTrans=!showTrans">修改密码</el-button>
            </el-form-item>
            <el-collapse-transition>
                <div v-if="showTrans">
                    <el-form-item label="密码" class="left-form-item" prop="password">
                        <el-input show-password placeholder="请输入账号密码" v-model="editForm.old_password"></el-input>
                    </el-form-item>
                    <el-form-item label="新密码" prop="password">
                        <el-input show-password placeholder="请输入新密码" v-model="infoForm.password"></el-input>
                    </el-form-item>
                    <el-form-item label="确认密码" prop="check">
                        <el-input show-password placeholder="请再次输入密码" v-model="infoForm.check"></el-input>
                    </el-form-item>
                </div>
            </el-collapse-transition>
            <el-form-item v-if="!showEdit" label="密码" prop="password">
                <el-input show-password placeholder="请输入密码" v-model="infoForm.password"></el-input>
            </el-form-item>
            <el-form-item v-if="!showEdit" label="确认密码" prop="check">
                <el-input show-password placeholder="请再次输入密码" v-model="infoForm.check"></el-input>
            </el-form-item>
            <el-form-item label="邮箱">
                <el-input placeholder="请输入邮箱" v-model="infoForm.email"></el-input>
            </el-form-item>
            <el-button v-if="this.type=='reg'" type="primary" v-on:click="register('infoForm')" class="register_button">注册</el-button>
            <el-link v-if="this.type=='reg'" :href="this.loginUrl" style="float:right;" :underline="false">已有账号？登录</el-link>
        </el-form>
</template>

<script>
export default {
    name:'profile-form',
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
            var passEmail=(rule,value,callback)=>{
                if ("\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*".test(value)){
                    callback();
                }
                else{
                    callback(new Error('请输入正确的邮箱地址'));
                }
            }
        return{
            title: this.type=='edit'?'资料编辑':'注 册',
            formClass: this.type=='edit'?'edit-form':'register-form',
            showEdit: this.type == 'edit'?true:false,
            showTrans: false,
            loginUrl:'/login',
            /*表单信息*/
            infoForm:{
                username:this.info.username,
                password:'',
                check:'',
                email:'',
            },
            editForm:{
                intro: '',
                avatar: '',
                old_password:'',
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
                    {required:true,validator:passCheck,trigger:'blur'},
                ]
            }
        }
    },
    props:{
        type:{
            type:String,
            default: 'edit',
        },
        info:{
            type: Object,
            default:function(){
                return{
                    username:'',
                    intro:'',
                    email:'',
                    avatar:'',
                }
            }
        }
    },
    methods:{
        /**注册方法 */
        register(formName){
            /*validate这是属于element-ui的方法 */
            this.$refs[formName].validate((valid)=>{
                //表单验证通过，向后端发送请求
                if(valid){
                    alert('submit');
                    let _this = this;
                    this.$axios.post('/reg',{
                        username: this.infoForm.username,
                        password: this.infoForm.password,
                        email: this.infoForm.email,
                    })
                    .then(scuccessResponse=>{
                        if (scuccessResponse && scuccessResponse.status == 200){
                            //后端通过请求并处理
                            if (scuccessResponse.data.code == 0){
                                _this.$store.commit('login',{
                                    username: this.infoForm.username,
                                    token: scuccessResponse.data.msg.token,
                                    id: scuccessResponse.data.msg.id,
                                });
                                _this.$router.push('/home');
                            }
                            //存在错误，后端拒绝处理
                            else{
                                this.$message({
                                    message: '信息存在问题，请重新尝试',
                                    type:'error',
                                    offset: 100,
                                })
                            }
                        }
                    })
                    .catch(failResponse=>{
                         _this.$notify.error({
                            title: '注册失败',
                            message: failResponse.data,
                        });
                    })
                }
                else{
                    this.$message({
                        message:'请确保信息正确！',
                        type: warnig,
                        offset:100,
                    })
                    return false;
                }
            });
        },
        /**更新*/
        updateInfo(formName){
          this.$refs[formName].validate((valid)=>{
            if(valid){
              alert('submit');
              let result = {
                intro: this.editForm.intro,
                email: this.infoForm.email,
                username: this.infoForm.username,
                
              };
              this.$axios.post('/user/update/info',resul).then((successResponse)=>{
                if (successResponse && successResponse.status == 200){
                  if (successResponse.data.code == 0){
                    this.$message({
                        message:'修改成功喵~',
                        type:'success',
                        offset:100,
                    })
                    //alert('success');
                    if ( !this.showTrans){
                        this.$emit('setVisiblity',false);
                        this.$router.go(0);
                    }
                  }
                  else{
                    this.$message.error(successResponse.data.msg);
                  }
                }
              }).catch((failResponse)=>{})
              if (this.showTrans){
                  this.updatePassword();
              }
            }
          })
        },
        /**修改密码 */
        updatePassword(){
            this.$axios.post('/user/update/pwd',{
                oldPwd: this.editForm.old_password,
                newPwd: this.infoForm.password,
            }).then((successResponse)=>{
                if (successResponse && successResponse.status == 200){
                    if(successResponse.data.code == 0){
                        this.$message({
                            message:'密码修改成功',
                            type:'success',
                            offset:100,
                        });
                        this.infoForm.password = '';
                        this.infoForm.check = '';
                        this.editForm.old_password = '';
                        this.$emit('setVisiblity',false);
                        this.$router.go(0);
                    }else{
                        this.$message({
                            message:successResponse.data.msg,
                            type:'error',
                            offset:100,
                        })
                    }
                }
            }).catch((failResponse)=>{})
        }
    },
    watch:{
        info:{
            handler(val,oldVal){
                this.editForm.intro = this.val.intro;
                this.editForm.avatar = this.val.avatar;
                this.infoForm.email = this.val.email;
                console.log('watch info change to ',val);
            },
            deep: true,
        }
    },
    mounted:function(){
        this.editForm.intro = this.info.intro;
        this.editForm.avatar = this.info.avatar;
        this.infoForm.email = this.info.email;
    }
}
</script>

<style scoped>
.register-form{
    border-radius: 15px;
    width: 350px;
    border: 1px solid #eaeaea;
    padding: 0;
    box-shadow: 0 0 25px #C0C4CC;
    background-color: rgba(255, 255, 255, 0.1);
}
.edit-form{
    border-radius: 15px;
    width: fit-content;
    padding: 15px 35px 15px 35px;
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
