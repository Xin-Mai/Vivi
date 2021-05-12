<template>
<el-dialog ref="profile"
    title="资料编辑"
  :visible.sync="dialogVisible"
  width="30%">
  <div class="avatar-upload-container" v-on:click="openUpload">
    <el-avatar :size="50" :src="avatar"></el-avatar>
    <i class="el-icon-camera-solid" :style="icon"></i>
  </div>
  <el-dialog
    width="35%"
      title="头像上传"
      :visible.sync="uploadVisible"
      append-to-body>
      <el-upload
        drag accept=".jpg,.jpeg,.png,.gif"
        list-type="picture"
        :auto-upload="false"
        :file-list="avatarList"
        :on-change="upload"
        :action="uploadUrl">
    <i class="el-icon-upload"></i>
    <div class="el-upload__text">将文件拖到此处，或<em>点击上传</em></div>
    <div class="el-upload__tip" slot="tip">只能上传jpg/png文件，且不超过1Mb</div>
    </el-upload>
    <span slot="footer" class="dialog-footer">
      <el-button @click="uploadVisible = false">取 消</el-button>
      <el-button type="primary" @click="uploadAvatar">上 传</el-button>
    </span>
    </el-dialog>
  <profile-form :type="'edit'"></profile-form>
  <span slot="footer" class="dialog-footer">
      <el-button @click="dialogVisible = false">取 消</el-button>
      <el-button type="primary" >确 认</el-button>
    </span>
</el-dialog>
</template>

<script>
import ProfileForm from './profileForm.vue'
import graphCoder from '@/assets/utils/graphCoder.js'
export default {
  components: { ProfileForm },
    name:'edit-dialog',
    data(){
        return{
            dialogVisible: false,
            uploadVisible: false,
            username:'',
            password:'',
            check:'',
            uploadUrl:'/api',
            defaultAvatar:"https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png",
            avatar:this.getAvatar,
            avatarList:[],
            dataUrl:'hhh'

        }
    },
    methods:{
      openUpload(){
        this.uploadVisible = true;
      },
      /*头像上传 */
      upload(file,fileList){
          this.avatarList[0] = file;
          console.log(fileList);
          if(fileList.length > 1){
            fileList.shift();
          }
          //this.codeGraph(file.raw);
        },
        /**将图片编码为base64格式，转为dataUrl */
      async codeGraph(file){
          let _this=this;
          graphCoder.codeGraph(file).then((result)=>{
            _this.dataUrl = result;
          });
      },
      //检查图片大小
      sizeCheck(file){
        return (file.size / 1024 / 1024 <= 1);
      },
      async uploadAvatar(){
          if (this.avatarList.length >0 ){
            //console.log('here');
            if (this.sizeCheck(this.avatarList[0])){
              await this.codeGraph(this.avatarList[0].raw);
              console.log(this.dataUrl);
              this.$axios.post('/user/update/avatar',this.dataUrl,{
            headers:{
              token:this.$store.state.token,
            }
          })
            .then(successResponse=>{
            if(successResponse && successResponse.status ==200){
              this.uploadVisible = false;
              this.avatar = this.dataUrl;
            }
          })
            .catch(failResponse=>{
            this.$message({
              massage:'上传失败',
              type:'error',
              offset:100,
            });
            
          })
            }
            else{
              this.$message.error('图片不能超过1Mb');
            }
        }
        else{
        this.$message({
              message:'请选择一张图片',
              type:'error',
              offset:100,
            })
      }
        //this.uploadVisible = false;
      }
    },
    computed:{
      icon(){
        return{
          position:'absolute',
          right:'-3px',
          bottom:0,
        }
      },
      getAvatar(){
        this.$axios.post('/avatar',{'id':this.$store.state.user.id},{
          headers:{
            token:this.$store.state.token,
          }
        })
        .then(successResponse=>{
          if(successResponse && successResponse.status ==200){
            console.log(successResponse.data.length);
            return successResponse.data;
          }
        })
        .catch(failResponse=>{
          return this.defaultAvatar;
        })
      }
    }
}
</script>
<style scoped>
.avatar-upload-container{
  position: relative;
  width: 50px;
  margin-left: 35px;
}
</style>