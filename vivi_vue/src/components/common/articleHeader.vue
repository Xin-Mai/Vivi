<template>
    <div class="header-container">
        <div class="top-buttons" v-if="is_mine">
            <el-button type="text" icon="el-icon-edit" v-on:click="updateArticle">修改</el-button>
            <el-button type="text" icon="el-icon-delete" style="color:gray" v-on:click="deleteArticle">删除</el-button>
        </div>
        <div id="background">   
            <div id="moon"></div>
            <h1 id="title">{{title}}</h1>
            <div id="mountain1"></div>
            <div id="mountain2"></div>
            <div class="detail">
                <i class="el-icon-date"></i>
                <label id="publish-date">{{publishDate}}</label>
                <i class="iconfont icon-yanjing"></i>
                <label id="read-num">{{readNum}}</label>
            </div>
        </div>
    </div>
</template>

<script>
export default{
    name:'article-header',
    data(){
        return{

        }
    },
    props:{
        is_mine:{
            type: Boolean,
            default: true,
        },
        title:{
            type: String,
            default: '测试标题',
        },
        publishDate:{
            type: String,
            default: 'today',
        },
        readNum:{
            type: Number,
            default: 1,
        }
    },
    methods:{
        updateArticle(){
            this.$parent.updateArticle();
        },
        deleteArticle(){
            this.$$parent.deleteArticle();
        }
    }
}

</script>

<style scoped>
.header-container{
    z-index: -4;
    width:100%;
    overflow-y: hidden;
    height: 400px;
}
#background{
    position: absolute;
    overflow: hidden;
    top:0;
    left: 0;
    width: 100%;
    height: 400px;
    z-index: -4;
    background:url('~@/assets/img/starnight.jpg') center;
    background-size: cover;
    /*background: linear-gradient(150deg,#70e1f5,#ffd194);*/
}
#moon{
    z-index: -4;
    background-image: url('~@/assets/img/moon.png') ;
    position: fixed;
    left: 10%;
    top:260px;
    background-size: cover;
    width:60px;
    height:60px;
    animation-duration: 300s;
    animation-iteration-count: infinite;
    animation-timing-function: linear;
    transform-origin: 500px 400px;
    animation-name: circle;
}
@keyframes circle {
  0% {
   transform: rotate(0deg);
  }
  100% {
    transform: rotate(90deg);
  }
}
#title{
    z-index: -1;
    color: white;
    position: fixed;
    top: 100px;
    font-size: 50px;
    right: 0;
    left: 0;
    margin: 0 auto;
    text-shadow: 3px 3px 9px #303133;
}
#mountain1{
    z-index: 0;
    position: absolute;
    top: -20px;
    left: 0;
    background-image: url('~@/assets/img/mountain1.png') ;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    height: 500px;
    overflow: hidden;
    width: 100%;
}
#mountain2{
    z-index: -2;
    position: absolute;
    top: 0px;
    left: 0;
    background-image: url('~@/assets/img/mountain2.png') ;
    background-position: top;
    background-repeat: no-repeat;
    background-size: cover;
    height: 400px;
    width: 100%;
}
.detail{
    text-align: end;
    font-size: 14px;
    color:#C0C4CC;
    position: absolute;
    bottom: 10px;
    right:40px;
}
#publish-date::before{
    content:'发布日期 '
}
#read-num::before{
    content: '阅读量 ';
}
.top-buttons{
    position: absolute;
    top: 10px;
    right: 30px;
}
</style>
