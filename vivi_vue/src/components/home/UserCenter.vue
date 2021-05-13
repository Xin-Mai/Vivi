<template>
    <div class="body">
        <el-button :v-show="passagerId==user.id?true:false"
        type="text" style="float:right"
        icon="el-icon-edit-outline" v-on:click="editProfile">编辑资料</el-button>
        <edit-dialog ref="dialog" v-bind="user"></edit-dialog>
        <div class="personal">
                <div class="avatar_container" >
                    <one-line-desc :size="'big'" :uid="user.id"></one-line-desc>
                </div>
                <el-button round  type="primary" class="button" icon="el-icon-plus"
                :disabled="passagerId==user.id?true:false"
                v-on:click="followChange">关注</el-button>
                <el-button round class="button" icon="el-icon-message">私信</el-button>
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
                    <div v-if="articles.length==0?true:false">还没有发表过文章哦</div>
                    <div v-else class="pane">
                        <item-card v-for = "i in articles.length" v-bind:key=i v-bind="articles[i-1]"></item-card>
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
import followAndMessage from '@/assets/utils/followAndMessage.js'
import avatarGetter from '@/assets/utils/avatarGetter.js'
import EditDialog from '../common/EditDialog.vue';
export default {
  components: { OneLineDesc, ItemCard, EditDialog },
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
            ],
            articles:[],
            user:{
                id: this.$route.params.id,
                username:'',
                intro:'',
                avatar:'',
            },
            passagerId: this.$store.state.user.id,
        }
    },
    methods:{
        handleClick(tab, event){
            console.log(tab,event);
        },
        followChange(){
            followAndMessage.followChange(this.user.id,this.passagerId);
        },
        /**显示资料编辑框 */
        editProfile(){
            this.$refs.dialog.dialogVisible = true;
        },
        getAvatar(){
            avatarGetter.getAvatar(this.user.id).then((url)=>{
                this.user.avatar = url;
            });
        }
    },

    created:function(){
        //加载头部信息
        this.$axios.post('/user',{'id':this.user.id})
        .then(successResponse=>{
            if (successResponse && successResponse.status == 200){
                if (successResponse.data.code == 0){
                    /*
                    // 取数据
                    this.nums[0] = successResponse.data.followingNum;
                    this.nums[1] = successResponse.data.followerNum;
                    this.nums[2] = successResponse.data.articleNum;
                    this.nums[3] = successResponse.data.readNum;
                    this.nums[4] = successResponse.data.likeNum;*/
                    //取简介的数据
                    this.user.username = successResponse.data.msg.username;
                    this.user.intro = successResponse.data.msg.intro;
                    this.user.email = successResponse.data.msg.email;
                }
            }
        })
        .catch(failResponse=>{

        })
        this.getAvatar();
        this.$axios.post('/article/user',{
            'id':this.user.id,
        })
        .then(successResponse=>{
            if (successResponse && successResponse.status == 200){
                if (successResponse.data.code == 0){
                    this.articles = successResponse.data.msg;
                }
                //console.log(this.articles);
            }
        })
        .catch(failResponse=>{
            this.$notify.error({
                title:'加载失败',
                message: '请稍后再试',
            })
        })
    }

}
</script>
<style>
.body{
    min-height: 100%;
}
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
