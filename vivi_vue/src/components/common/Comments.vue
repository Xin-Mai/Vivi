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
            <div v-else>
            <comment-card v-for="i in data.length" :key="i" v-bind="commentList[i-1]"></comment-card>
            <el-pagination 
                small
                style="padding:20px" 
                @current-change="handelCurrentChange"
                :page-size="pageSize"
                :current-page="currentPage"
                :hide-on-single-page="true"
                layout="prev, pager, next"
                :total="commentList.length"
                >
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
            passagerId: this.$store.state.id,
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
                return [
                    {content:'hh'},
                    {content:'xixi'},
                    {content:'233'},
                    {content:'wow'},
                    {content:'wwwww'},
                    {content:'666'}]
            }
        },
        aid:{
            type: Number,
            default: 0,
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
        publishComment(){
            if (!this.passagerId){
                this.$message({
                    message: '请先登录再发表评论哦',
                    offset: 200,
            });
                return;
            }
            this.showHint = this.input.length==0?true:false;
            if (!this.showHint){
                /**向后端发送请求,添加评论 */
                this.$axios.post('/addComment',{
                    content: this.input,
                    from: this.passagerId,
                    to: this.aid,
                })
                .then(successResponse=>{
                    if (successResponse && successResponse.status==200){
                        this.commentList.append({content: this.input});
                        this.input = '';
                    }
                })
                .catch(failResponse=>{
                    this.$message({
                        message:'评论失败，请稍后再试',
                        type:error,
                        offset:200,
                    })
                })
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