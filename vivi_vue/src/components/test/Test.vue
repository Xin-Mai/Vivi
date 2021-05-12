<template>
<div>
    <h1>{{rsp}}</h1>
    <p>{{data}}</p>
</div>
</template>

<script>
export default {
    name:'Test',
    data(){
        return{
            rsp:'',
            data:'',
        }
    },
    created:function(){
        //this.$store.commit('logout');
        this.$axios.get()
        .then(successRsp=>{
            if (successRsp && successRsp.status == 200){
                this.rsp = successRsp.data;
            }
        }).catch(failRsp=>{

        });
        
        this.$axios.get('/hello',{
            headers:{
                'token': this.$store.state.token,
            }
        })
        .then(successRsp=>{
            if (successRsp && successRsp.status == 200){
                this.data = successRsp.data;
            }
        }).catch(failRsp=>{

        });
        
    }
}
</script>