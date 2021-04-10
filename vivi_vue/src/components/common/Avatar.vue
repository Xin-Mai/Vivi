<template>
    <el-dropdown>
        <el-avatar :size="medium" :src="circleUrl"></el-avatar>
        <el-dropdown-menu slot="dropdown">
            <el-dropdown-item>个人中心</el-dropdown-item>
        </el-dropdown-menu>
    </el-dropdown>
    
</template>

<script>
export default {
    name:'my-avatar',
    data(){
        return{
            circleUrl: "https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png",
        }
    }
    
}
</script>

<style scoped>

</style>