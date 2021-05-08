<template>
    <div class="comment-container">
        <one-line-desc :descContent="from"></one-line-desc>
        <div>{{content}}</div>
        <el-button type="text" class="reply">回复</el-button>
    </div>
    
</template>
<script>
import OneLineDesc from './OneLineDesc.vue'
export default {
  components: { OneLineDesc },
    name:'comment-card',
    data(){
        return{

        }
    },
    props:{
        publishDate:{
            type: String,
            default: new Date().toString(),
        },
        content:{
            type: String,
            default:'',
        },
        from:{
            type: Object,
            default:function(){
                return{
                    id: '',
                    username: '',
                    url: '',
                    intro: this.publishDate,
                }
            }
        },
        commentList:{
            type: Array,
            default:function(){
                return []
            }
        }
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
</style>