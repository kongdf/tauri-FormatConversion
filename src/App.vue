<template>
  <el-config-provider :locale="zhCn" size="default">
    <el-container>
      <el-aside :style="{ width: state.isCollapse ? '65px' : '220px' }">
        <el-menu default-active="2" :popper-effect="'dark'" :collapse="state.isCollapse" router>
          <el-menu-item index="home">
            <el-icon>
              <HomeFilled />
            </el-icon>
            <template #title>首页</template>
          </el-menu-item>
          <el-menu-item index="tovideo">
            <el-icon>
              <VideoCamera />
            </el-icon>
            <template #title>视频转换</template>
          </el-menu-item>
          <el-menu-item index="tomp3">
            <el-icon>
              <Headset />
            </el-icon>
            <template #title>音频转换</template>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-main> <router-view /></el-main>
    </el-container>
  </el-config-provider>
</template>
<script setup>
import zhCn from 'element-plus/lib/locale/lang/zh-cn'

const state = reactive({
  isCollapse: false
})
 
</script>

<style lang="scss">
html,
body {
  height: 100%;
  width: 100%;
}

* {
  margin: 0;
  padding: 0;
}

#app {
  height: 100%;
}

.el-container {
  height: 100%;
}

.el-menu {
  height: 100%;
}
</style>