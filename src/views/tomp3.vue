<template>
   <div>
      <el-form label-width="160px">
         <el-form-item label="请选择输出格式:">
            <el-radio-group v-model="state.format" class="ml-4">
               <el-radio label="MP3">MP3</el-radio>
               <el-radio label="WMA">WMA</el-radio>
               <el-radio label="M4A">M4A</el-radio>
               <el-radio label="WAV">WAV</el-radio>
               <el-radio label="MP2">MP2</el-radio>
               <el-radio label="Flac">Flac</el-radio>
            </el-radio-group>
         </el-form-item>
      </el-form>

      <el-upload ref="up" drag multiple :auto-upload="false" v-show="!state.isRunFFMPEG" :limit="1"
         :show-file-list="false" :on-change="RunFFMPEG">
         <el-icon class="el-icon--upload"><upload-filled /></el-icon>
         <div class="el-upload__text">拖拽文件到此处或点击选择文件</div>
      </el-upload>

      <div style="text-align: center;" v-show="state.isRunFFMPEG">

         <el-progress type="circle" :percentage="state.progress" :status="state.progress == 100 ? 'success' : ''" />
         <div style="margin-top: 20px;" v-show="state.progress == 100">
            <el-button type="primary" @click="rest">返回</el-button>
         </div>
      </div>


   </div>
</template>

<script setup>
import { UploadFilled } from "@element-plus/icons-vue";
import { createFFmpeg, fetchFile } from "@ffmpeg/ffmpeg";
import { useRouter } from 'vue-router';
const router = useRouter()
const state = reactive({

   format: 'MP3',
   isRunFFMPEG: false,
   progress: 0,
   isShow: false,
   message: "",
   message2: ''
});

const rest = () => {
   state.isRunFFMPEG = false
   up.value.clearFiles()
   state.progress = 0
}

const up = ref(null)


const RunFFMPEG = async (res) => {
   console.log(res)
   state.isRunFFMPEG = true;

   const { name } = res.raw;
   let downName = name.substring(0, name.lastIndexOf(".")) + '.' + state.format

   const ffmpeg = createFFmpeg({
      log: false,
      progress: ({ ratio }) => {
         state.progress = parseInt((ratio * 100))
      },
   });
   await ffmpeg.load();

   ffmpeg.FS("writeFile", name, await fetchFile(res.raw));

   await ffmpeg.run("-i", name, downName);

   const data = ffmpeg.FS("readFile", downName);
   downLoadFileBlob(data.buffer, downName);
};

const downLoadFileBlob = (data, fileName) => {
   const blob = new Blob([data], { type: "charset=utf-8" });
   const file = new FileReader();
   file.readAsText(blob, "utf-8");
   file.onload = function () {
      state.isShow = false;
      const URL = window.URL || window.webkitURL

      const fileUrl = URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = fileUrl;
      link.download = fileName;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
   };
};
</script>