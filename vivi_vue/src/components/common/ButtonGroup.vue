<template>
    <ul class="button-group" >
        <li>
            <div class="radius" v-if="is_like==1"></div>
            <el-button id="like" v-on:click="like" 
            :style="buttonColor"
            class="button" circle 
            :icon="is_like==0?unlike_icon:liked_icon" ></el-button>
            <div class="num" 
            >{{realLike}}</div>
        </li>
        <li>
            <el-button  id="comment"
            class="button" circle 
            v-on:click="scrollToComment" 
            icon="iconfont icon-pinglun"></el-button>
            <div class="num">{{commentNum}}</div>
        </li>
    </ul>
</template>

<script>
export default {
    name:'button-group',
    data(){
        return{
            is_like:this.ILike,
            realLike: this.likeNum,
            unlike_icon:"iconfont icon-aixin",
            liked_icon:"iconfont icon-aixin1",
            aid: this.$route.params.id,
        }
    },
    props:{
        ILike:{
            type: Boolean,
            default: false,
        },
        likeNum:{
            type: Number,
            default: 0,
        },
        commentNum:{
            type: Number,
            default: 0,
        },
        commentX:{
            type: Number,
            default: 0,
        },
        commentY:{
            type: Number,
            default: 0,
        }
    },
    methods:{
        like(){
            if (this.$store.state.user.id == ''){
                this.$message({
                    message:'请先登录',
                    offset: 100,
                });
            }else{
                this.is_like = !this.is_like;
                if (this.is_like){
                    this.realLike += 1;
                    
                }else{
                    this.realLike -= 1;
                    document.getElementById("like").blur();
                }
            }
            /**向后端发请求 */
            //this.$axios.post('/article/like',this.aid);
        },
        scrollToComment(){
            //console.log('click scroll');
            document.getElementById("comment").blur();
            window.scrollTo(this.commentX,this.commentY);
        },
        likePost(){
            if (this.ILike != this.is_like){
                this.$axios.post('/article/like',{
                    id: this.aid,
                }).catch(failRsp=>{

                })
            }
        }
    },
    computed:{
        buttonColor(){
            if (this.is_like == 1){
                return "color:red;"
            }
        },
    },
    watch:{
        ILike(val, oldVal){
            console.log('ILike changed from ',oldVal,' to ',val);
            this.is_like = val;
        },
        likeNum(val, oldVal){
            console.log('LikeNum changed from ',oldVal,' to ',val);
            this.realLike = val;
        },
    },
    created:function(){
        //console.log('commentNum',this.commentNum);
    },
    mounted:function(){
        if (this.ILike){
            this.is_like = 1;
        }
        else{
            this.is_like = 0;
        }
    },
    beforeDestroy:function(){
        this.likePost();
    }
    
}
</script>

<style scoped>
.button-group{
    list-style: none;
    width: fit-content;
    padding: 0;
}
.num{
    color: #606266;
    margin-top:5px;
}
.button-group li{
    position: relative;
    display: block;
    height: 80px;
}
@keyframes radius {
    0%{  transform: scale(1,1);
        opacity: 0.6;
        }
    100%{
        transform: scale(1.5,1.5);
        opacity: 0;
    }
}
.radius{
    border:2px solid red;
    z-index:-1;
    position: absolute;
    top:3px;
    left:calc(50% - 22px);
    border-radius: 50%;
    height: 40px;
    width: 40px;
    animation: radius 0.5s;
    animation-iteration-count: 2;
    animation-play-state: running;
    animation-delay: 0;
}
.button{
    width: 50px;
    height: 50px;
    margin: 0 auto;
    box-shadow: 0 0 15px #C0C4CC;
    z-index: 2;
}
button:hover{
    background: white;
}
button:focus{
    background: white;
    border:none;
}
/**抖动的动画 */
@keyframes shaking /* Safari 与 Chrome */
{
    0% {
        transform:translate(0px, 0px) rotate(0deg);
    }
    25% {
        transform:translate(-1px, 1.5px) rotate(15deg);
    }
    50% {
        transform:translate(1.3px, 0px) rotate(-15deg);
    }
    75% {
        transform:translate(1.4px, 1.4px) rotate(15deg);
    }
    100% {
        transform:translate(0px, 0px) rotate(0deg);
    }
}
#like:hover{
    color:red;
    animation: shaking 0.5s;
    animation-iteration-count: infinite;
    animation-play-state: running;
    animation-delay: 0;
}
#like:focus{
    color:red;
}


</style>