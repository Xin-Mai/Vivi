<template>
    <el-card :body-style="{ padding: '0px' }" class="card">
        <div class="item">
        <one-line-desc id="desc" :uid="uid"></one-line-desc>
        <div class="middle">
            <div class="animation-container">
                <el-link class="title" :underline="false" :href="'/article/'+id" >{{this.title}}</el-link>
                <div class="content">{{this.content}}</div>
            </div>
        </div>
        <div class="info">
            <label class="label1"><i class="iconfont icon-yanjing"></i>{{this.readNum}}</label>
            <label class="label2"><i class="iconfont icon-aixin"></i>{{this.likeNum}}</label>
        </div>
        </div>
    </el-card>
</template>

<script>
import OneLineDesc from './OneLineDesc'
export default {
    name:'item-card',
    components:{
        OneLineDesc
    },
    props:{
        id:{
            type: String,
            default:"",
        },
        uid:{
            type: String,
            default:"",
        },
        title:{
            type:String,
            default:'Hello,world'
        },
        content:{
            type:String,
            default:'content here!\ntest\ntest\ntest\ntest\ntest\ntest\ntest\ntest\ntest'
        },
        readNum:{
            type:Number,
            default:0
        },
        likeNum:{
            type:Number,
            default:0
        },
        author:{
            type: Object,
            default:function(){
                return{
                    name:'',
                    intro:'',
                    avatar: "https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png",
                }
            }
        }
    },
    data(){
        return{
            
        }
        
    },
    mounted(){
    }
}
</script>
<style scoped>
.card{
    width: 200px;
    max-width: 200px;
    height: 300px;
    margin: 15px 10px;
    --main-color:#C0C4CC;
    --main-hover-color:white;
    --content-color:#909399;
}
.item{
    width: 200px;
    height: 300px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    background:url("~@/assets/img/cbg3.jpeg");
    background-size: cover;
    background-position: center;
}
#desc{
    width:90%;
    margin-top:10px;
    --name-color:var( --main-color);
    --name-hover-color:var( --main-hover-color);
}
.middle{
    height: 80%;
    margin: 0 5px 0 5px;
    /*width: 100%;*/
    overflow: hidden;
}
.animation-container{
    width: 100%;
    height: 100%;
    transform: translateY(84px);
    transition: transform 3s;
    transition-timing-function: cubic-bezier(0.075, 0.82, 0.165, 1);
}
.animation-container:hover{
    transform: translateY(10%);

}
.title{
    font-size: 18px;
    font-weight: bold;
    color: var(--main-color);
    margin-bottom: 5px;
}
.title:hover{
    color: var(--main-hover-color);
}
.content{
    height: 40%;
    line-height: 16px;
    font-size: 14px;
    color:var( --content-color);
    text-overflow: ellipsis;
    white-space: pre-line;


}
.info{
    width: 100%;
    height: 30px;
    font-size: 13px;
    color: var( --main-color);
    text-align: right;
    position: relative;
}
.label2{
    position:absolute;
    bottom:10px;
    right: 10px;
}
.label1{
    position:absolute;
    bottom:10px;
    right:45px;
}
</style>