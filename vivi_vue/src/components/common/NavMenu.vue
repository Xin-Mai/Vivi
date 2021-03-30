<template>
    <el-menu :default-active="activeIndex" mode="horizontal" :router="is_router" @select="handleSelect"
    background-color="#545c64" text-color="#fff" active-text-color="#ffd04b" class="menu">
        <label class="title">VIVI</label>
        <el-menu-item :index="activeIndex">首页</el-menu-item>
        <el-menu-item :index="followIndex">关注</el-menu-item>
        <el-input placeholder="请输入内容" 
        prefix-icon="el-icon-search" v-model="searchContent" size="medium" class="searchbox">
        <el-button slot="append" icon="el-icon-search"></el-button> 
         </el-input>
    </el-menu>
</template>

<script>
export default {
    name:'nav-menu',
    data(){
        return{
            activeIndex:'/index',
            followIndex:'/follow',
            searchContent:'',
            is_router:true,
        }
    },
    methods:{
        handleSelect(key,keyPath){
            console.log(key,keyPath);
        }
    }
}
</script>

<style scoped>
    .menu{
        display:flex;
        align-items: center;
        flex-direction:row;
        z-index: 100;
    }
    .title{
        display: inline;
        height: fit-content;
        font-size: 40px;
        font:bold;
        color: ivory;
        margin-inline-start: 100px;
        margin-inline-end: 100px;
    }
    .searchbox{
        width: 300px;
        margin-inline-start: 100px;
    }
</style>>

