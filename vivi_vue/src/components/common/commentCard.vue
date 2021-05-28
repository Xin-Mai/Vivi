<template>
    <div class="comment-container">
        <div class="comment-body" :data-cid="cid">
            <one-line-desc :uid="from.id" :type="'comment'" :publishDate="publishDate" ref="commenter"></one-line-desc>
            <div>{{content}}</div>
            <el-button type="text" class="reply" v-if="!typing" data-type="reply">回复</el-button>
        </div>
        <el-collapse-transition>
            <div class="reply-input" v-if="typing">
                <el-input placeholder="请输入回复" v-model="input"></el-input>
                <div class="buttons" :data-cid="cid">
                    <label class="hint" v-show="showHint">评论不能为空</label>
                    <el-button round size="mini" type="info" plain data-type="cancle">取消</el-button>
                    <el-button round size="mini" type="primary" v-on:click="reply" data-type="publish">发布</el-button>
                </div>
            </div>
        </el-collapse-transition>
        <div class="reply-list">
            <div v-show="!showReply">
                <reply-card v-for="i in replyBrower.length" :key="i"
                v-bind="replyBrower[i-1]"></reply-card>
                <div v-if="replyList.length>3&&!showReply?true:false" class="reply-hint">
                    {{replyHint}}
                    <el-button type="text" v-on:click="showReply=true">点击查看</el-button>
                </div>
            </div>
            <div v-show="showReply">
                <reply-card v-for="i in replyShowed.length" :key="i" v-bind="replyShowed[i-1]"></reply-card>
                <el-pagination
                    small
                    :total="replyList.length"
                    :page-size="pageSize"
                    :current-page="currentPage"
                    :hide-on-single-page="true"
                    @current-change="handelCurrentChange"
                    layout="prev, pager, next">
                    </el-pagination>
            </div>
        </div>
    </div>
    
</template>
<script>
import OneLineDesc from './OneLineDesc.vue'
import ReplyCard from './replyCard.vue';
export default {
  components: { OneLineDesc,ReplyCard },
    name:'comment-card',
    data(){
        return{
            typing: false,
            input:'',
            showHint: false,
            from:{
                id: this.uid,
                username: '',
                avatar: '',
                intro: this.publishDate,
            },
            showReply: false,
            currentPage: 1,
            pageSize: 5,
        }
    },
    props:{
        cid:{
            type: String,
            default: "",
        },
        quote:{
            type: String,
            default: "",
        },
        uid:{
            type: String,
            default: "",
        },
        publishDate:{
            type: String,
            default: new Date().toString(),
        },
        content:{
            type: String,
            default:'',
        },
        replyList:{
            type: Array,
            default:function(){
                return [];
            }
        }
    },
    methods:{
        cancelTyping(){
            this.input = "";
            this.showHint = false;
            this.typing = false;
        },
        reply(){
            this.showHint = this.input.length==0?true:false;
            if (!this.showHint){
                this.$emit('commentTo',this.cid,this.input);
            }
        },
        /**控制翻页 */
        handelCurrentChange(val){
            this.currentPage = val;
        }
    },
    computed:{
        replyHint(){
            return "共"+this.replyList.length+"条回复，";
        },
        //用于预览的部分
        replyBrower(){
            if (this.replyList.length <=3 ){
                return this.replyList;
            }
            return this.replyList.slice(0,3);
        },
        //用于展示的部分
        replyShowed(){
            if ((this.currentPage)*this.pageSize>this.replyList.length){
                return this.replyList.slice((this.currentPage-1)*this.pageSize);
            }
            return this.replyList.slice((this.currentPage-1)*this.pageSize,(this.currentPage)*this.pageSize);
        }
    },
    watch:{
        cid(val, oldVal){
            //console.log("cid changed from ",oldVal," to ",val);
            //document.getElementsByClassName("reply").id = this.cid;
        }
    },
    mounted:function(){
        //console.log(this.cid);
        //document.getElementsByClassName("reply").id = this.cid;
        //console.log(document.getElementsByClassName("reply"));
    },
}
</script>

<style scoped>
.comment-container{
    padding:20px;
    border-bottom: 1px solid #E4E7ED;
}
.comment-body{
    position: relative;
}
.reply{
    position: absolute;
    padding: 0;
    right: 20px;
    bottom:0px;
}
.reply-input{
    margin-top: 25px;
}
.buttons{
    margin-top: 10px;
    text-align: right;
}
.hint{
    float: left;
    font-size:14px;
    color:red;
}
.reply-list{
    margin: 20px 0 0 42px;
    
}
.reply-hint{
    text-align: start;
    font-size: 13px;
}
.el-button--text{
    font-size: 13px;
}
</style>