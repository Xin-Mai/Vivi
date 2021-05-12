<template>
    <div :style="styleVar" class="one-line-desc-container">
        <div class="avatar-container">
            <el-avatar :src="data.avatar"  class="avatar"></el-avatar>
        </div>
        <div class="desc">
            <el-link class="name" :underline="false">{{data.username}}</el-link>
            <label class="intro">{{data.intro}}</label>
        </div>
    </div>
</template>

<script>
export default {
    name:'one-line-desc',
    data(){
        return{
            defaultUrl: "https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png",
            defaultIntro: "没有简介",
        }
    },
    props:{
        uid:{
            type: String,
            default: "",
        },
        descContent:{
            type: Object,
            default:function(){
                return{
                    id:'',
                    username: 'hh',
                    intro: this.defaultIntro,
                    avatar: this.defaultUrl,
                }
            }
        },
        size:{
            type:String,
            default:'small',
        },
        font_size:{
            type:Number,
            default:16,
        },
        avatar_size:{
            type:Number,
            default:36,
        },
        intro_font_size:{
            type:Number,
            default:14,
        }
    },
    computed:{
        data(){
            /**通过uid从后端拿数据 */
            if (this.uid ){
                let author;
                this.$axios.get('/user/'+this.uid)
                .then(successResponse=>{
                    if (successResponse && successResponse.status==200){
                        return successResponse.data;
                    }
                });
            }
            /**直接传值而不是uid */
            return {
                    id: this.descContent.id,
                    username: this.descContent.username,
                    avatar: this.url,
                    intro: this.intro,
            }
            
        },
        url(){
            if (!this.descContent.avatr){
                return this.defaultUrl;
            }
            return this.descContent.avatar;

        },
        intro(){
            if(!this.descContent.intro){
                return this.defaultIntro;
            }
            return this.descContent.intro;
        },
        styleVar(){
            //console.log(this.size);
            let fontSize = this.font_size;
            let introFontSize = this.intro_font_size;
            let avatarSize = this.avatar_size;
            if (this.size == 'big'){
                fontSize = 20;
                introFontSize = 16;
                avatarSize = 60;
            }
            return{
                '--name-font-size': fontSize + 'px',
                '--avatar-size': avatarSize + 'px',
                '--intro-font-size': introFontSize +'px',
            }
        }
    },
    created:function(){
        //console.log(this.descContent);
    }

}
</script>

<style scoped>
/*字体颜色*/
.one-line-desc-container{
    width: fit-content;
    min-width: 150px;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: center;
    --name-color:#606266;
    --intro-color:#909399;
    --name-hover-color:black;
}
.avatar-container{
    height:var(--avatar-size);
}
.avatar{
    height: var(--avatar-size);
    width:var(--avatar-size) ;
    line-height: var(--avatar-size);
}
.name{
    font-size: var(--name-font-size);
    color:var( --name-color);
}
.name:hover{
    color: var(--name-hover-color);
}
.intro{
    font-size: var(--intro-font-size);
    color:#909399;
    overflow: hidden;
    height: calc(var(--intro-font-size) + 4px);
}
.desc{
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    width: 60%;
    height:var(--avatar-size);
    margin-left: 10px;
}
</style>