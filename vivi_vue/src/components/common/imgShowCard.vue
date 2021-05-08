<template>
    <div class="container">
        <el-card class="card">           
            <el-image v-show="show"
            style="width: 100px; height: 100px;"
            :src="imgList[this.id].url"></el-image>           
        </el-card>
        <el-pagination  layout="prev, pager, next" :total="imgNum" :page-size="1" :current-page="id+1" @current-change="changeHandeler"></el-pagination>
    </div>
</template>
<script>
import img1 from '@/assets/img/bg.jpg'
import img2 from '@/assets/img/bg3.jpg'
import img3 from '@/assets/img/bg4.jpg'
export default {
    name:'show-card',
    data(){
        return{
            imgNum:3,
            imgList:[
                {url:img1},
                {url:img2},
                {url:img3},
            ],
            id:0,
            show:true
        }
    },
    created(){
        this.autoChange()
    },
    methods:{
        autoChange(){
            setInterval(()=>{
                //this.show=!this.show;
                this.id=(this.id+1)%this.imgList.length;
                //this.show=true;
            },3000)
        },
        changeHandeler(current){
            if(current==this.id+1)
                return;
            this.id=current-1;
        }
    }
}
</script>
<style scoped>
.container{
    display:flex ;
    flex-direction: column;
    align-items: center;
}
.card{
    width: 400px;
}

</style>