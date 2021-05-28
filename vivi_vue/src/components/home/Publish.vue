<template>
    <div class="container">
        <el-input class="input"
        v-model="title" 
        placeholder="请输入标题" 
        maxlength="30"
        suffix-icon="el-icon-edit"></el-input>
        <el-input class="input" 
        v-model="content"
        placeholder="请输入内容" 
        type="textarea" 
        rows="20" 
        maxlength="5000" show-word-limit></el-input>
        <div class = "button-group">
            <el-button class="button" v-on:click="cancel" type="info" plain circle icon="el-icon-close"></el-button>
            <el-button class="button" v-on:click="commit" type="primary" icon="el-icon-edit" circle></el-button>
        </div>
    </div>
</template>

<script>
export default {
    name:'Publish',
    data(){
        return{
            aid: this.$route.query.aid ,
            title:'',
            content:'',
            tag:'',
        }
    },
    methods:{
        cancel(){
            this.$confirm('取消发表内容将不作保存，是否继续？','提示',{
                confirmButtonText: '确定',
                cancelButtonText: '取消',
                type: 'info'
            }).then(()=>{
                this.$router.go(-1);
            }).catch(()=>{

            });
        },
        commit(){
            if (this.title&&this.content){
                //console.log(this.title,this.content);
                if (!this.aid){
                    this.aid = "";
                }
                this.$axios.post('/article/publish',
                {
                    title: this.title,
                    content: this.content,
                    tag: this.tag,
                    id: this.aid,
                })
                .then(successResponse=>{
                    if(successResponse && successResponse.status==200){
                        if (successResponse.data.code == 0){
                            this.$message({
                            message:'发表成功',
                            type: 'success',
                            offset:100,
                            });
                            if (successResponse.data.msg != ''){
                                this.$router.push('/article/'+successResponse.data.msg);
                            }else{
                                this.$router.push('/article/'+this.aid);
                            }
                        }
                    }
                }).catch(failResponse=>{
                    this.$message({
                            message:'发表失败，请稍后再试',
                            type: 'error',
                            offset:100,
                        });
                })

            }
            else{
                this.$message({
                    message:'标题与内容不能为空',
                    type:'warning',
                    offset:100,
                });
            }
        }
    },
    created:function(){
        console.log(this.$route);
        if (this.$route.params){
            let params = this.$route.params;
            console.log(params);
            this.title = this.$route.params.title;
            this.content = this.$route.params.content;
            this.aid = this.$route.params.aid;
        }
    }
}
</script>

<style scoped>
.container{
    position:absolute;
    bottom: 0;
    left: 0;
    right: 0;
    margin: 0 auto;
}
.input{
    margin:10px;
    width: 80%;
}
.button-group{
    position: absolute;
    right: 50px;
    bottom:100px;
}
.button{
    display: block;
    margin: 10px;
    width: 50px;
    height: 50px;
}
</style>