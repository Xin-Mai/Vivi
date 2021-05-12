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
            <div v-else @click="handleReply">
                <comment-card v-for="i in data.length" :key="i" v-bind="data[i-1]" :ref="data[i-1].cid"></comment-card>
                <el-pagination id="p"
                    small
                    style="padding:20px" 
                    @current-change="handelCurrentChange"
                    :page-size="pageSize"
                    :current-page="currentPage"
                    :hide-on-single-page="true"
                    layout="prev, pager, next"
                    :total="commentList.length">
                </el-pagination>
            </div>
        </div>
</div>
</template>
<script>
import commentCard from './commentCard.vue';
export default {
  components: { commentCard },
    name:'comments',
    data(){
        return{
            input:'',
            passagerId: this.$store.state.user.id,
            showHint: false,
            typing:false,
            currentPage:1,
            pageSize:5,
        }
    },
    props:{
        commentList:{
            type: Array,
            default:function(){
                return []
            }
        },
        aid:{
            type: String,
            default: "",
        }
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
                        updateComment({
                            cid: successResponse.data.cid,
                            uid: this.passagerId,
                            publishDate: successResponse.data.publishDate,
                            quote:val,
                            content: val,
                            });
                        this.input = '';
                        this.$message({
                            message:'评论成功！',
                            type:'success',
                            offset:100,
                        })
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
            /**判断是否是回复按钮 */
            if (e.target.parentElement.nodeName.toLowerCase() == 'button'){
                console.log(e.target.parentElement.id);
                /**是回复 */
                /**修改子组件数据 */
                let cid = e.target.parentElement.id;
                this.$refs[cid][0].typing = true;
                console.log(this.$refs[cid][0]);
            }
        },
        /**对评论数据进行更新 */
        updateComment(comment){
            //回复评论的
            if (comment.quote){
                for (let i = (this.currentPage-1)*this.pageSize;i<this.commentList.length;i++){
                    if (this.commentList[i].cid == comment.quote){
                        this.commentList[i].replyList.push(comment);
                    }
                }
            }
            else{
                this.commentList.push(comment);
            }

        }
    }
}
</script>

<style scoped>
/**评论区标题字体 */
.container{
    width:96%;
    margin: 0 auto;
}
.header{
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-left: 20px;
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