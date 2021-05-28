<template>
    <div class="container">
        <article-header style="z-index:-1" 
        :publishDate="article.publishDate" 
        :readNum="article.readNum"
        :title="article.title"></article-header>
        <div class="content">
            <el-card class="author-card" shadow="hover" >
                <one-line-desc :size="'big'" :uid="article.uid"></one-line-desc>
                <div style="margin-top:15px;text-align:left">
                    <el-button class="button" round type="primary" 
                    :disabled="is_mine"
                    size="small" icon="el-icon-plus" 
                    v-on:click="followChange">关注</el-button>
                    <el-button class="button" round size="small" icon="el-icon-message">私信</el-button>
                </div>
            </el-card>
            {{article.content}}</div>
            <div class="comment"><comments  id="commentArea"></comments></div>
        <button-group class="buttons" :ILike="article.like" :likeNum="article.likeNum"
		:commentX="commentX" :commentY="commentY" :commentNum="commentNum"></button-group>
    </div>
</template>

<script>
import ArticleHeader from '../common/articleHeader.vue'
import ButtonGroup from '../common/ButtonGroup.vue';
import Comments from '../common/Comments.vue';
import OneLineDesc from '../common/OneLineDesc.vue';
import followAndMessage from '@/assets/utils/followAndMessage.js'
import dateFilter  from '@/assets/utils/dateFilter.js'
export default {
  components: { ButtonGroup, OneLineDesc, Comments, ArticleHeader },
    name:'Article',
    data(){
        return{
            article:{
                aid:this.$route.params.id,
                title:'this is a title',
                content:'this is content\ntest',
                publishDate: new Date().toString(),
                likeNum: 0,
                readNum: 0,
                uid: '',
                like: false,
            },
            commentList: [
                    /*{content:'hh',cid:'a'},
                    {content:'xixi',cid:'b'},
                    {content:'233',cid:'c'},
                    {content:'wow',cid:'d'},
                    {content:'wwwww',cid:'e'},
                    {content:'666',cid:'f'}*/
                    ],
            commentNum: 0,
            readerId: this.$store.state.user.id,
            commentX: 0,
            commentY: 0,
        }
    },
    methods:{
        followChange(){
            followAndMessage.followChange(this.article.author.uid, this.$store.state.user.uid);
        },
        //将获取的评论构造成树形结构
        dealComments(rawData){
            let commentList = [];
            let replyMap = [];
            for (let item of rawData){
                //是评论的回复
                if (item.quote){
                    if (replyMap.get(quote)){
                        replyMap.get(quote).push(item); 
                    }
                    else{
                        replyMap.set(quote, new Array().push(item));
                    }
                }
                else{
                    commentList.push(item);
                }
            }
            for (let item of commentList){
                item.repliList = replyMap.get(item.cid);
            }
            return commentList;
        },
        /**修改文章 */
        updateArticle(){
            console.log(this.article);
            this.$router.push({
                name: 'Publish',
                path: '/publish',
                params: {
                    title: this.article.title,
                    content: this.article.content,
                    aid: this.article.aid,
                },
            })
        },
        /**删除文章 */
        deleteArticle(){
            if (!this.article.aid){
                this.article.aid = window.location.href.replace(/.*article\//g,'');
            }
            //console.log(this.article.aid);
            this.$confirm('此操作将永久删除该文章, 是否继续?', '提示', {
                confirmButtonText: '确定',
                cancelButtonText: '取消',
                type: 'warning'
                }).then(() => {
                    this.$axios.post('/article/delete',{
                        id: this.article.aid,
                    },{
                        headers:{
                            token: this.$store.state.token
                        }
                    }).then(successResponse=>{
                        if (successResponse && successResponse.status == 200){
                            if (successResponse.data.code == 0){
                                this.$message({
                                    type: 'success',
                                    message: '删除成功!'
                                });
                                this.$router.push('/usercenter/'+this.article.uid);
                            }else{
                                this.$message({
                                    type: 'error',
                                    message: successResponse.data.msg,
                                });
                            }
                        }
                    }).catch(failResponse=>{
                        this.$message({
                                    type: 'error',
                                    message: '删除失败，请稍后再试',
                                });
                    });
                }).catch(() => {
                    this.$message({
                    type: 'info',
                    message: '已取消删除'
                    });          
            });
        },
        /**获取评论数 */
        setCommentNum(val){
            console.log(val);
            this.commentNum = val;
        }
    },
    watch:{
        article:{
            handler(val,oldVal){
                console.log('article changed from ',oldVal,' to ',val);
            },
            deep: true,
        }
    },
    created:function(){
        let aid = this.article.aid;
        this.$axios.post('/article',{'id':this.article.aid})
        .then(successResponse=>{
            if (successResponse && successResponse.status==200){
                this.article = successResponse.data.msg;
                this.article.aid = aid;
            }
        })
        .catch(failResponse=>{
            this.$message({
                message: '加载失败，请刷新再试',
                type:'error',
                offset:100,
            });
        });
    },
    //阅读量增加
    mounted:function(){
        this.article.readNum += 1;
        let commentArea = document.getElementById('commentArea');
        this.commentX = commentArea.getBoundingClientRect().x;
        this.commentY = commentArea.getBoundingClientRect().y;
    },
    computed:{
        is_mine(){
            return this.$store.state.user.id==this.article.uid;
        },
    },
    watch:{
        article:{
            handler(val, oldVal){
                if (val.publishDate){
                    val.publishDate = dateFilter.filt(val.publishDate);
                }
            },
            deep: true,
        },
    }
}
</script>
<style scoped>
.container{
    z-index: 0;
    background: white;
    height: 100%;
    width: 80%;
}
.button{
    width:45%;
}
/**内容 */
.content{
    z-index: 1;
    background: white;
    white-space: pre-line;
    width: 100%;
    text-align: left;
    margin: 0 auto;
    padding: 24px 0;
    min-height: 200px;
}
.buttons{
    position: fixed;
    left: 2%;
    top:50%;
}
.author-card{
    float:left;
    width:240px;
    min-width: 240px;
    margin:0px 10px 10px 0px;
}
.comment{
    padding:0 20px 0 20px;
    background: white;
    z-index: 1;
}
</style>