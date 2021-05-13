<template>
  <el-dialog title="资料编辑" :visible.sync="dialogVisible" width="30%">
    <div class="avatar-upload-container" v-on:click="openUpload">
      <el-avatar :size="50" :src="updateForm.avatar"></el-avatar>
      <i class="el-icon-camera-solid" :style="icon"></i>
    </div>
    <el-dialog
      width="35%"
      title="头像上传"
      :visible.sync="uploadVisible"
      append-to-body
    >
      <el-upload
        drag
        accept=".jpg,.jpeg,.png,.gif"
        list-type="picture"
        :auto-upload="false"
        :file-list="avatarList"
        :on-change="upload"
      >
        <i class="el-icon-upload"></i>
        <div class="el-upload__text">将文件拖到此处，或<em>点击上传</em></div>
        <div class="el-upload__tip" slot="tip">
          只能上传jpg/png文件，且不超过1Mb
        </div>
      </el-upload>
      <span slot="footer" class="dialog-footer">
        <el-button @click="uploadVisible = false">取 消</el-button>
        <el-button type="primary" @click="uploadAvatar">上 传</el-button>
      </span>
    </el-dialog>
    <profile-form :type="'edit'" :info="updateForm" ref="infoForm"></profile-form>
    <span slot="footer" class="dialog-footer">
      <el-button @click="dialogVisible = false">取 消</el-button>
      <el-button type="primary" v-on:click="updateInfo">确 认</el-button>
    </span>
  </el-dialog>
</template>

<script>
import ProfileForm from "./profileForm.vue";
import graphCoder from "@/assets/utils/graphCoder.js";
import avatarGetter from "@/assets/utils/avatarGetter.js";
export default {
  components: { ProfileForm },
  name: "edit-dialog",
  data() {
    return {
      dialogVisible: false,
      uploadVisible: false,
      uid: this.$store.state.user.id,
      check: "",
      updateForm:{
        username:this.$store.state.user.username,
        avatar:'',
        intro:'',
      },
      avatarList: [],
      dataUrl: "hhh",
    }
  },
  props:{
    intro:{
      type: String,
      default:''
    },
    email:{
      type: String,
      default: '',
    }
  },
  methods: {
    openUpload() {
      this.uploadVisible = true;
    },
    /*头像上传 */
    upload(file, fileList) {
      this.avatarList[0] = file;
      console.log(fileList);
      if (fileList.length > 1) {
        fileList.shift();
      }
      //this.codeGraph(file.raw);
    },
    //检查图片大小
    sizeCheck(file) {
      return file.size / 1024 / 1024 <= 1;
    },
    async uploadAvatar() {
      if (this.avatarList.length > 0) {
        //console.log('here');
        if (this.sizeCheck(this.avatarList[0])) {
          graphCoder.codeGraph(this.avatarList[0].raw).then((res) => {
            this.dataUrl = res;
            this.$axios
              .post("/user/update/avatar", res, {
                headers: {
                  token: this.$store.state.token,
                },
              })
              .then((successResponse) => {
                if (successResponse && successResponse.status == 200) {
                  this.uploadVisible = false;
                  this.avatar = this.dataUrl;
                }
              })
              .catch((failResponse) => {
                this.$message({
                  massage: "上传失败",
                  type: "error",
                  offset: 100,
                });
              });
          });
        } else {
          this.$message.error("图片不能超过1Mb");
        }
      } else {
        this.$message({
          message: "请选择一张图片",
          type: "error",
          offset: 100,
        });
      }
      //this.uploadVisible = false;
    },
    updateInfo() {
      //console.log('updateInfo')
      //console.log(this.$refs.infoForm);
      this.$refs.infoForm.updateInfo('infoForm');
      this.dialogVisible = false;
      this.$router.go(0);
    },
  },
  computed: {
    icon() {
      return {
        position: "absolute",
        right: "-3px",
        bottom: 0,
      };
    }
  },
  /**因为prop是异步的，所以要监听避免data获取不了Prop */
  watch:{
    intro(value,oldValue){
      this.updateForm.intro = value;
      //console.log("intro change from"+oldValue+'to'+value);
    }
  },
  created:function(){
    avatarGetter.getAvatar(this.uid).then((url)=>{this.updateForm.avatar = url;});
    console.log(this.updateForm);
  }
};
</script>
<style scoped>
.avatar-upload-container {
  position: relative;
  width: 50px;
  margin-left: 35px;
}
</style>
