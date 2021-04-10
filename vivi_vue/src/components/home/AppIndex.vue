<template>
    <div>
        <el-backtop target=".page-component__scroll .el-scrollbar__wrap"></el-backtop>
        <show-card></show-card>
        <div class="main-container">
            <div class="content-list">
                <item-card v-for="i in contentList.length" v-bind:key="i"></item-card>
            </div>
            <div class="recommend-list">
                <el-card class="card">
                    <div class="recommend-header">推荐作者</div>
                    <one-line-desc v-for="i in recommendList.length" v-bind:key="i"></one-line-desc>
                </el-card>
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
            contentList:[{a:1}],
            recommendList:[{a:1}],
        }
    },
    components:{
        ShowCard,
        ItemCard,
        OneLineDesc
        },
}
</script>

<style scoped>
.main-container{
    display: flex;
    flex-direction: row;
    width: 100%;
}
.content-list{
    width: 70%;
}
.recommend-list{
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 30%;
    padding:5px;
    background-color: black;
}
.card{
    width: 80%;

}
.recommend-header{
    font-size: 14px;
    color:#909399;
    text-align: left;
    padding-bottom: 10px;
}
</style>
