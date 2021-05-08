<template>
    <div>
        <show-card></show-card>
        <div class="main-container">
            <div class="content-list">
                <item-card v-for="i in contentList.length" v-bind:key="i"></item-card>
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
            contentList:[{a:1},{a:1},{a:1},{a:1},{a:1},{},{},{},{},{}],
            recommendList:[{a:1}],
        }
    },
    components:{
        ShowCard,
        ItemCard,
        OneLineDesc
        },
    created:function(){
        
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
    margin:20px 10px;
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
