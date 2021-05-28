<template>
    <div>
        <show-card></show-card>
        <div class="main-container">
            <div class="content-list">
                <item-card v-for="i in contentList.length" v-bind:key="i"
                v-bind="contentList[i-1]"></item-card>
            </div>
            <div class="side-bar">
                <div class="list">
                    <el-card class="list-card">
                        <div class="list-header">全站阅读量排名</div>
                    </el-card>
                </div>
                <div class="list">
                    <el-card class="list-card">
                        <div class="list-header">推荐作者</div>
                        <one-line-desc v-for="i in recommendList.length" v-bind:key="i"></one-line-desc>
                    </el-card>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import ShowCard from '../common/imgShowCard'
import ItemCard from '../common/itemCard.vue'
import OneLineDesc from '../common/OneLineDesc'
export default {
    name:'AppIndex',
    data(){
        return{
            contentList:[{a:1},],
            recommendList:[{a:1}],
        }
    },
    components:{
        ShowCard,
        ItemCard,
        OneLineDesc
        },
    created:function(){
        
    },
    mounted:function(){
        this.$axios.get('/article/all')
        .then(successResponse=>{
            if (successResponse && successResponse.status == 200){
                if (successResponse.data.code == 0){
                    this.contentList = successResponse.data.msg;
                }else{
                    this.$message({
                        message: '加载失败，请稍后再试',
                        type:'error',
                        offset: 100,
                    })
                }
            }
        }).catch(failResponse=>{
            this.$message({
                message: failResponse.data,
                type:'error',
                offset: 100,
            })
        })
    }
}
</script>

<style scoped>
.main-container{
    width: fit-content;
    margin: 0 auto;
    display: flex;
    flex-direction: row;
}
.content-list{
    width: 900px;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    justify-content: flex-start;
    flex-shrink: 1;
}
.side-bar{
    float: right;
    margin-right:10%;
    width: 240px;
}
.list{
    width: 100%;
    margin-bottom:20px;
    flex-shrink: 0;
}
.list-card{
    width: 100%;
    border-radius: 0px;
}
.list-header{
    font-size: 14px;
    color:#909399;
    text-align: left;
    padding-bottom: 10px;
}
</style>
