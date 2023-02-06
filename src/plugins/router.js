import { createRouter, createWebHistory } from "vue-router";

// 路由信息
const routes = [

    {
        path: "/home",
        name: "home",
        component: () => import("../views/home.vue"), 
    }, {
        path: "/tomp3",
        name: "tomp3",
        component: () => import("../views/tomp3.vue"), 
    },{
        path: "/tovideo",
        name: "tovideo",
        component: () => import("../views/tovideo.vue"), 
    },
];

// 导出路由
const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
