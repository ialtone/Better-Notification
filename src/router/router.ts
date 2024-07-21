import { createRouter, createWebHashHistory } from 'vue-router'
const routes = [
    {
        path: '/',
        component: () => import('../pages/style1/style1.vue'),
    }
];

const router = createRouter({
    routes,
    history: createWebHashHistory(),
})


export default router