<template>
    <el-dropdown @command="handleCommand">
        <el-avatar :size="size" src="https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png"></el-avatar>
        <el-dropdown-menu slot="dropdown">
            <div style="text-align:center">{{username}}</div>
            <el-dropdown-item command='user_center'>个人中心</el-dropdown-item>
            <el-dropdown-item command="log">{{botton}}</el-dropdown-item>
        </el-dropdown-menu>
    </el-dropdown>
    
</template>

<script>
export default {
    name:'my-avatar',
    data(){
        return{
            size: "medium",
            botton: "",
            login_botton: "登录",
            logout_botton: "退出",
        }
    },
    created: function(){
        //console.log("created")
        var user = this.$store.state.user;
        if (user.username){
            this.botton = this.logout_botton;
            this.username == user.username;
        }
        else{
            this.botton = this.login_botton;
        }
    },
    methods:{
        handleCommand(command){
            if ( command=='user_center'){
                console.log('click user center');
                var user = this.$store.state.user;
                if (user.id != ""){
                    this.$router.push({path:'/usercenter/'+user.id});
                }
                else{
                    this.$notify.error({
                        title: '请先登录',
                        message: '登录后才能访问个人中心'
                    });
                }
            }
            else{
                if (command == 'log'){
                    console.log(this.botton);
                    if (this.botton == this.login_botton){
                        this.$router.push({path:'/login'});
                    }
                    else{
                        this.botton = this.login_botton;
                        this.logout();
                        //this.$router.go(0);
                    }
                }
            }
        },
        logout(){
            this.$store.commit('logout');
            /**console.log(this.$store.state.user.id);
            this.$axios.get('/logout/'+this.$store.state.user.id)
            .then(successResponse=>{
                this.$store.commit('logout');
                console.log(this.$store.state.user);
                //this.$router.go(0);
            })
            .catch(failResponse=>{
                this.$notify.error({
                    titie:'退出失败',
                    message:'请重新尝试',
                })
            })*/
        }
    },
    computed:{
        username(){
            if (this.$store.state.user.username != "")
                return this.$store.state.user.username;
            else{
                return "游客";
            }
        }
    }   
}
</script>

<style scoped>

</style>