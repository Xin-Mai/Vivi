<template>
    <div class="container">
        <el-avatar :src="url" :size="small" style="margin-end:10px"></el-avatar>
        <div class="desc">
            <el-link class="name" :underline="false">{{name}}</el-link>
            <label class="intro">{{introduction}}</label>
        </div>
    </div>
</template>

<script>
export default {
    name:'one-line-desc',
    data(){
        return{
            url:"https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png",
            name:'hh',
            introduction:'hh',
        }
    }
}
</script>

<style scoped>
.name{
    font-size: 16px;
}
.name:hover{
    color: black;
}
.intro{
    font-size: 14px;
    color:#909399;
}
.container{
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    margin-top: 5px;
}
.desc{
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    width: 60%;
}
</style>