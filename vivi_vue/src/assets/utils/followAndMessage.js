export default{
    followChange( author, follower){
        if (!follower){
            this.$message({
                message:'请先登录',
                offeset:100,
                type:warn,
            });
        }
        let params = new URLSearchParams();
        params.append('author',author);
        params.append('follower',follower);
        this.$axios.post('/followChange',params)
        .then(successReponse=>{
            if (successReponse && successReponse.status == 200){
                return successReponse.data.state;
            }
        })
        .catch(failResponse=>{
            this.$message({
                message: '操作失败，请稍后再试',
                type: error,
                offeset: 100,
            })
        })
    },
    sendMessage(from, to ,message){
        
    }
}