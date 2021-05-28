<template>
<div class="container">
        <el-divider><i class="el-icon-chat-line-square"></i></el-divider>
        <div class="header">
            <h3>评论区</h3>
            <div class="comment-area">
                <el-input v-model="input" v-on:click.native="isTyping"
                class="comment-input" 
                type="textarea" :autosize="{ minRows: 3}"
                suffix-icon="el-icon-edit"
                placeholder="对作者说点什么吧？"></el-input>
                <el-collapse-transition>
                <div class="buttons" v-show="typing">
                    <label class="hint" v-show="showHint">评论不能为空</label>
                    <el-button round size="mini" type="info" plain v-on:click="cancelTyping">取消</el-button>
                    <el-button round size="mini" type="primary" v-on:click="publishComment">发布</el-button>
                </div>
                </el-collapse-transition>
            </div>
        </div>
        <div class="comment-list">
            <div v-if="commentList.length==0?true:false" style="padding:20px">
                <el-divider>还没有人评论哦，沙发就决定是你啦！</el-divider>
            </div>
            <div v-else @click="handleReply($event)">
                <comment-card v-for="i in data.length" :key="i" :cid="data[i-1]._id" :data-cid="data[i-1]._id"
                v-bind="data[i-1]" :ref="data[i-1]._id"></comment-card>
                <el-pagination id="p"
                    small
                    style="padding:20px" 
                    @current-change="handelCurrentChange"
                    :page-size="pageSize"
                    :current-page="currentPage"
                    :hide-on-single-page="false"
                    layout="prev, pager, next"
                    :total="commentList.length">
                </el-pagination>
            </div>
        </div>
</div>
</template>
<script>
import commentCard from './commentCard.vue';
import dateFilter from '@/assets/utils/dateFilter.js'
export default {
  components: { commentCard },
    name:'comments',
    data(){
        return{
            aid: this.$route.params.id,
            commentList:[],
            input:'',
            passagerId: this.$store.state.user.id,
            showHint: false,
            typing:false,
            currentPage:1,
            pageSize:5,
        }
    },
    props:{
        comments:{
            type: Array,
            default:function(){
                return []
            }
        },
    },
    computed:{
        data(){
            if ((this.currentPage)*this.pageSize>this.commentList.length){
                return this.commentList.slice((this.currentPage-1)*this.pageSize);
            }
            return this.commentList.slice((this.currentPage-1)*this.pageSize,(this.currentPage)*this.pageSize);
        }
    },
    methods:{
        isTyping(){
            this.typing = true;
        },
        cancelTyping(){
            this.input = '';
            this.showHint = false;
            this.typing = false;
        },
        handelCurrentChange(val){
            this.currentPage = val;
        },
        /**评论请求 */
        commentTo(cid, val){
            /**向后端发送请求,添加评论 */
                this.$axios.post('/comment/publish',{
                    content: val,
                    quote: cid,
                    aid: this.aid,
                })
                .then(successResponse=>{
                    if (successResponse && successResponse.status==200){
                        if (successResponse.data.code == 0){
                            if (cid != ''){
                                this.$refs[cid][0].input = '';
                                this.$refs[cid][0].typing = false;
                                
                            }else{
                                this.input = '';
                            }
                            this.updateComment({
                                _id: successResponse.data.msg._id,
                                uid: this.passagerId,
                                publishDate: successResponse.data.msg.publishDate,
                                quote:cid,
                                content: val,
                                });
                            this.$message({
                                message:'评论成功！',
                                type:'success',
                                offset:100,
                            })
                        }
                    }
                })
                .catch(failResponse=>{
                    this.$message({
                        message:'评论失败，请稍后再试',
                        type:'error',
                        offset:200,
                    })
                })
        },
        /**对文章进行评论 */
        publishComment(){
            if (this.passagerId == ""){
                this.$message({
                    message: '请先登录再发表评论哦',
                    offset: 200,
            });
                return;
            }
            this.showHint = this.input.length==0?true:false;
            if (!this.showHint){
                this.commentTo("",this.input);
            }
        },
        /**回复评论,使用事件委派 */
        handleReply(e){
            let target;
            //判断是否是按钮
            if (e.target.nodeName.toLowerCase() == 'span'){
                target = e.target.parentElement;
            }else{
                if (e.target.nodeName.toLowerCase() == 'button'){
                    target = e.target;
                }
            }
            if (target){
                //是按钮
                let cid = target.parentElement.dataset.cid;
                console.log(target.parentElement.dataset);
                let component = this.$refs[cid][0];
                if (target.dataset.type == 'reply'){
                    //console.log(this.$refs); 
                    component.typing = true;
                }
                else if (target.dataset.type == 'cancle'){
                    component.typing = false;
                }
                else if (target.dataset.type == 'publish')
                {
                    component.showHint = component.input.length==0?true:false;
                    if (!component.showHint){
                        this.commentTo(cid, component.input);
                    }
                }
            }
        },
        /**对评论数据进行更新 */
        updateComment(comment){
            //回复评论的
            comment.publishDate = dateFilter.filt(comment.publishDate);
            if (comment.quote){
                let flag = 0;
                for (let i = (this.currentPage-1)*this.pageSize;i<this.commentList.length;i++){
                    if (this.commentList[i]._id == comment.quote){
                        this.commentList[i].replyList.push(comment);
                        flag = 1;
                    }
                    else{
                        for (let reply of commentList[i].replyList){
                            if (reply._id == comment.quote){
                                flag = 1;
                                break;
                            }
                        }
                    }
                    if (flag == 1){
                        break;
                    }
                }
            }
            else{
                this.commentList.push(comment);
            }

        },
        /**将评论变为二层树状结构 */
        commentFilter(comments){
            console.log(comments);
            let replyList = [];
            for (let item of comments){
                item.publishDate = dateFilter.filt(item.publishDate);
                //是回复文章的评论
                if (!item.quote){
                    item.replyList = [];
                    this.commentList.push(item);
                }
                else{
                    replyList.push(item);
                }
            }
            //整理出回复别人的回复，将同一个评论下的所有回复放入一个列表
            for (let i=0;i<replyList.length;i++){
                for (let comment of this.commentList){
                    let flag = 0;
                    //是回复评论的回复
                    if (replyList[i].quote == comment._id){
                        comment.replyList.push(replyList[i]);
                        //console.log('add ',replyList[i],' to ',comment);
                        //找到了就无需继续往下看
                        flag = 1;
                    }
                    //查看是否是该评论的回复的回复
                    else{
                        for (let reply of comment.replyList){
                            if (replyList[i].quote == reply._id){
                                comment.replyList.push(replyList[i]);
                                flag = 1;
                                break;
                            }
                        }
                    }
                    if (flag == 1){
                        //这个回复已经找到所属的评论
                        break;
                    }
                }
            }
            console.log(this.commentList);
            this.$parent.setCommentNum(comments.length);
        }
    },
    mounted:function(){
        if (this.comments.length!=0){
            this.commentList = this.comments;
        }else{
            this.$axios.post('/comment',{
                id: this.aid
            }).then(successResponse=>{
                if (successResponse && successResponse.status == 200){
                    if (successResponse.data.code == 0){
                        //this.commentList = successResponse.data.msg;
                        this.commentFilter(successResponse.data.msg);
                    }
                }
            }).catch(failResponse=>{
                this.$message({
                    message: '评论加载失败',
                    type: 'error',
                    offset: 100,
                })
            })
        }
    },
    watch:{
    }
}
</script>

<style scoped>
/**评论区标题字体 */
.container{
    width:100%;
}
.el-divider{
    height: 1px;
}
.el-divider--horizontal{
    margin: 0;
}
.header{
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-left: 20px;
    margin-top: 24px;
}
.container h3{
    color:#606266;
    margin-block-start: 0;
}
.comment-area{
    flex-grow: 2;
    width: 80%;
}
.comment-input{
    width: 80%;
    margin: 0 auto;
}
/*因为是内部类，所以需要穿透 */
.el-textarea /deep/ .el-textarea__inner{
    background-color:#f8f8f8;
}
.buttons{
    margin-top: 10px;
    text-align: right;
    padding-right: 10%;
    padding-left: 10%;
}
.hint{
    float: left;
    font-size:14px;
    color:red;
}
</style>