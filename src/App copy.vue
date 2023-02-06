<template>
  <el-upload drag multiple :auto-upload="false" :on-change="chan">
    <el-icon class="el-icon--upload"><upload-filled /></el-icon>
    <div class="el-upload__text">拖拽文件到此处或点击选择文件</div>
  </el-upload>
  <div>
    {{ state.message2 }}
  </div>
  <div>
    {{ state.message }}
  </div>
</template>
<script setup>
import { UploadFilled } from "@element-plus/icons-vue";
import  { createFFmpeg,fetchFile } from "@ffmpeg/ffmpeg";
// const { createFFmpeg,fetchFile } = FFmpeg;

const state = reactive({
  isShow: false,
  message: "",
  message2:''
});

if (crossOriginIsolated) {
  state.message='可以'
  // Post SharedArrayBuffer
} else {
  // Do something else
  state.message='不可以'
}

if (window.isSecureContext) {
  // 页面在安全上下文中，所以 service worker 可用
  state.message2='在安全上下文中'
}


// const { createFFmpeg, fetchFile } = FFmpeg;

const chan = async (res) => {
  console.log(res);
  state.isShow = true;
  const ffmpeg = createFFmpeg({
    log: true,
    progress: ({ ratio }) => {
      state.message = `Complete: ${(ratio * 100.0).toFixed(2)}%`;
    },
  });
  const { name } = res.raw;

  state.message = "Loading ffmpeg-core.js";
  await ffmpeg.load();

  state.messageL = "Start transcoding";
  ffmpeg.FS("writeFile", name, await fetchFile(res.raw));
  await ffmpeg.run("-i", name, "output.mp3");
  state.message = "Complete transcoding";

  const data = ffmpeg.FS("readFile", "output.mp3");
  console.log(data);
  downLoadFileBlob(data.buffer, "output.mp3");
};

const downLoadFileBlob = (data, fileName) => {
  const blob = new Blob([data], { type: "charset=utf-8" });
  const file = new FileReader();
  file.readAsText(blob, "utf-8");
  file.onload = function () {
    // 若返回的是不是json对象，进行下载操作
    state.isShow = false;
    const fileUrl = window.URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = fileUrl;
    link.download = fileName;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };
};
</script>
<style scoped></style>
