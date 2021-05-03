<template>
    <div class="body">
        <div class="personal">
                <div class="avatar_container" >
                    <one-line-desc :size="'big'"></one-line-desc>
                </div>
                <el-button round  type="primary" class="button">关注</el-button>
                <el-button round class="button">发私信</el-button>
                <div class="info">
                <ul>
                    <li v-for="i in this.block_name.length" :key=i class="meta_block">
                        <a><p>{{nums[i-1]}}</p>{{block_name[i-1]}}</a>
                    </li>
                </ul>
                </div>
        </div>
        <el-divider></el-divider>
        <div class="article_container">
            <el-tabs v-model="activeName" @tab-click="handleClick" style="float:left;width:80%">
                <el-tab-pane label="文章" :lazy="is_lazy">
                    <div class="pane">
                        <item-card></item-card>
                    </div>
                </el-tab-pane>
                <el-tab-pane label="动态" :lazy="is_lazy">
                    <div class="pane">
                        <item-card></item-card>
                        <item-card></item-card>
                    </div>
                </el-tab-pane>
                <el-tab-pane label="热门" :lazy="is_lazy">
                    <div class="pane">
                        <item-card></item-card>
                    </div>
                </el-tab-pane>
            </el-tabs>
            <div class="list">
                    <el-card class="list-card">
                        <div class="list-header">创建的专题</div>
                        <ul>
                            <li>无</li>
                        </ul>
                    </el-card>
                </div>
        </div>
  </div>

</template>
<script>
import ItemCard from '../common/itemCard.vue';
import OneLineDesc from '../common/OneLineDesc.vue';

export default {
  components: { OneLineDesc, ItemCard },
    name:'UserCenter',
    data(){
        return{
            activeName:'0',
            is_lazy:true,
            block_name:[
                '关注',
                '粉丝',
                '文章',
                '阅读量',
                '喜欢'
            ],
            nums:[
                0,
                0,
                0,
                0,
                0,
            ]
        }
    },
    methods:{
        handleClick(tab, event){
            console.log(tab,event);
        }
    }
    
}
</script>
<style>
.el-tabs__item {
  font-size: 16px !important;
  padding: 10px 0px 40px !important;
  width: 120px !important;
}
.personal{
    width: fit-content;
    padding: 20px 0 10px 10px;
}
ul{
    padding-inline-start:10px;
    margin-block-start: 0;
    margin-block-end: 0px;
}
.personal ul li p{
    font-size: 16px;
    margin-block-end: 0;
    margin-block-start: 0;
}
.one-line-desc-container{
    --name-font-size:20px;
}
.avatar_container{
    display: inline-block;
}
.button{
    display: inline-block;
    margin-block-start: 10px;
    vertical-align: top;
}
.info{
    display: block;
    width: fit-content;
    height: fit-content;
    margin-block-start: 20px;
}
.meta_block{
    display: inline-block;
    font-size: 12px;
    margin: 0 10px 0 0;
    padding: 0 10px 0 0;
    border-right: 1px solid #f0f0f0;
}
.list{
    width: 200px;
    margin-bottom:20px;
    flex-shrink: 0;
    float: right;
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
/**清除浮动 */
.article_container::after{
    content: "";
    clear: both;
    display: block;
}
.pane{
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    flex-wrap: wrap;
}
</style>