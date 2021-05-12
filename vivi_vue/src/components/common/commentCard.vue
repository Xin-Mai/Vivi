<template>
    <div class="comment-container">
        <one-line-desc :descContent="from"></one-line-desc>
        <div>{{content}}</div>
        <el-button :id="cid" type="text" class="reply" v-if="!typing">回复</el-button>
        <el-collapse-transition>
            <div class="reply-input" v-if="typing">
                <el-input placeholder="请输入回复" v-bind="input"></el-input>
                <div class="buttons">
                    <label class="hint" v-show="showHint">评论不能为空</label>
                    <el-button round size="mini" type="info" plain v-on:click="cancelTyping">取消</el-button>
                    <el-button round size="mini" type="primary" v-on:click="reply">发布</el-button>
                </div>
            </div>
        </el-collapse-transition>
    </div>
    
</template>
<script>
import OneLineDesc from './OneLineDesc.vue'
export default {
  components: { OneLineDesc },
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
            }
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
    },
    mounted:function(){
        this.$axios.get('/user/'+this.uid)
        .then(successResponse=>{
            if (successResponse && successResponse.status == 200){
                this.from.username = successResponse.data.msg.username;
                this.from.avatar = successResponse.data.msg.avatar;
            }
        })
        .catch(failResponse=>{
            
        })
    }
}
</script>

<style scoped>
.comment-container{
    position: relative;
    padding:20px;
    border-bottom: 1px solid #E4E7ED;
}
.reply{
    position: absolute;
    right: 20px;
    bottom:5px;
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
</style>