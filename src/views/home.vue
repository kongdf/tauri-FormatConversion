<template>
    <div>
        home
        {{state.message   }}


        {{state.message2   }}
    </div>
</template>

<script setup>
const state=reactive({
    message:'不可以',
    message2:'不在'
})
if (crossOriginIsolated) {
    state.message = '可以'
    // Post SharedArrayBuffer
} else {
    // Do something else
    state.message = '不可以'
}

if (window.isSecureContext) {
    // 页面在安全上下文中，所以 service worker 可用
    state.message2 = '在安全上下文中'
}

</script>