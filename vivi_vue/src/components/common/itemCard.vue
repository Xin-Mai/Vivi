<template>
    <div class="item">
        <el-link class="title" :underline="false" >{{this.title}}</el-link>
        <div class="content">{{this.content}}</div>
        <div class="info">
            <label>{{this.author}}</label>
            <label>{{this.commentNum}}</label>
            <label>{{this.likeNum}}</label>
        </div>
        <div class="line"></div>
    </div>
</template>

<script>
export default {
    name:'item-card',
    props:{
        title:{
            type:String,
            default:'Hello,world'
        },
        content:{
            type:String,
            default:'content here!'
        },
        author:{
            type:String,
            default:'xin'
        },
        commentNum:{
            type:Number,
            default:0
        },
        likeNum:{
            type:Number,
            default:0
        }
    },
    data(){
        return{
            
        }
        
    }
}
</script>
<style scoped>
.item{
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    width: 600px;
    padding: 10px;
    margin: 20PX auto;
}
.title{
    font-size: 18px;
    font-weight: bold;
}
.title:hover{
    color: black;
}
.content{
    font-size: 14px;
    color:#909399;
}
.info{
    margin-top: 10px;
    font-size: 13px;
    color: #C0C4CC;
}
.line{
    margin-top: 10px;
    height: 1px;
    width:100%;
    background-color: #dccfe6;
}
</style>