<template>
    <div class="reply-container">
        <one-line-desc style="float:left"
        :type="'comment'" :publishDate="publishDate" :uid="uid"></one-line-desc>
        <el-link v-if="quoteUsername==''?false:true"
        :underline="false" class="quote">{{quoteUsername}}</el-link>
        <div class="content-container">
            <p class="content">{{content}}</p>
            <el-button type="text" class="reply-button">回复</el-button>
        </div>
    </div>
</template>
<script>
import OneLineDesc from './OneLineDesc.vue'
export default{
    name: 'reply-card',
    components:{
        OneLineDesc
    },
    data(){
        return{

        }
    },
    props:{
        username: {
            type: String,
            default: '已注销',
        },
        uid: {
            type: String,
            default: '',
        },
        content: {
            type: String
        },
        publishDate: {
            type: String,
            default: '刚刚'
        },
        quote:{
            type: String,
            default:'',
        },
        quoteUsername:{
            type: String,
            default: '',
        }
    },
    methods:{
       
    },
    mounted:function(){
            console.log(this.publishDate);
    }

}
</script>

<style scoped>
.reply-container{
    width: 100%;
    min-height: fit-content;
    padding: 5px 0 0 0;
    position: relative;
    border-top: 1px solid #E4E7ED;
}
.quote{
    font-size: var(--name-font-size);
}
.quote::before{
    content: '回复';
}
.content-container{
    text-align: start;
    min-height: 36px;;
    padding-right: 20px;
    font-size: 13px;
    position: relative;
    
}
.content{
    vertical-align: bottom;
}
.content::before{
    content:':';
}
.reply-button{
    position: absolute;
    bottom: 5px;
    right: 20px;
    padding:0;
}
.el-button--text{
    font-size: 13px;
}
</style>>
