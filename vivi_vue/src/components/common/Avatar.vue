<template>
    <el-dropdown @command="handleCommand">
        <el-avatar :size="size" src="https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png"></el-avatar>
        <el-dropdown-menu slot="dropdown">
            <el-dropdown-item command='user_center'>个人中心</el-dropdown-item>
        </el-dropdown-menu>
    </el-dropdown>
    
</template>

<script>
export default {
    name:'my-avatar',
    data(){
        return{
            size:"medium",
        }
    },
    methods:{
        handleCommand(command){
            if ( command=='user_center'){
                console.log('click user center');
                var user = this.$store.state.user.username;
                this.$router.push({path:'/usercenter',query:{username:user}});
            }
        }
    }   
}
</script>

<style scoped>

</style>