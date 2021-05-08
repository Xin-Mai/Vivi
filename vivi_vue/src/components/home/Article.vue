<template>
    <div class="container">
        <div class="header">
            <el-card class="author-card" shadow="hover">
                <one-line-desc :size="'big'"></one-line-desc>
                <div style="margin-top:15px;text-align:left">
                    <el-button class="button" round type="primary" 
                    :disabled="readerId == article.author.id?true:false"
                    size="small" icon="el-icon-plus" 
                    v-on:click="followChange">关注</el-button>
                    <el-button class="button" round size="small" icon="el-icon-message">私信</el-button>
                </div>
            </el-card>
            <div class="title-header">
                <h1 class="title">{{article.title}}</h1>
                <div class="detail">
                    <label id="publish-date">{{article.publishDate}}</label>
                    <i class="iconfont icon-yanjing"></i>
                    <label id="read-num">{{article.readNum}}</label>
                </div>
            </div>
        </div>
        <div class="content">{{article.content}}</div>
        <comments :aid="id"></comments>
        <button-group class="buttons"></button-group>
    </div>
</template>

<script>
import ButtonGroup from '../common/ButtonGroup.vue';
import Comments from '../common/Comments.vue';
import OneLineDesc from '../common/OneLineDesc.vue';
import followAndMessage from '@/assets/utils/followAndMessage.js'
export default {
  components: { ButtonGroup, OneLineDesc, Comments },
    name:'Article',
    data(){
        return{
            article:{
                id: this.$route.params.id,
                title:'this is a title',
                content:'this is content\ntest',
                publishDate: new Date().toString(),
                likeNum: 0,
                readNum: 0,
                author:{
                    id: '',
                    username:'',
                    intro:'',
                    avater:''
                }
            },
            readerId: this.$store.state.user.id,
        }
    },
    methods:{
        followChange(){
            followAndMessage.followChange(this.article.author.id, this.$store.state.user.id);
        }
    },
    created:function(){
        this.$axios.get('/article/'+this.$route.params.id)
        .then(successResponse=>{
            if (successResponse && successResponse.status==200){
                this.article = successResponse.data;
            }
        })
        .catch(failResponse=>{
            this.$message({
                message: '加载失败，请刷新再试',
                type: 'error',
                offset:100,
            });
        })
    },
    //阅读量增加
    mounted:function(){
        this.article.readNum += 1;
    },
    computed:{
    }
}
</script>
<style scoped>
.container{
    background: white;
    height: 100%;
    width: 80%;
}
/*头部*/
.header{
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: flex-start;
    padding: 20px 10px 20px 10px;
}
.button{
    width:45%;
}
.title-header{
    flex-grow: 2;
    padding:20px;
}
.title{
    margin-block-start:0px;
    text-align: left;
}
/**内容 */
.content{
    white-space: pre-line;
    width: 60%;
    text-align: center;
    margin: 0 auto;
    min-height: 200px;
}
.buttons{
    position: fixed;
    left: 2%;
    top:50%;
}
.author-card{
    width:240px;
    min-width: 240px;
}
.detail{
    text-align: end;
    font-size: 14px;
    color:#909399
}
#publish-date::before{
    content:'发布日期 '
}
#read-num::before{
    content: '阅读量 ';
}
</style>