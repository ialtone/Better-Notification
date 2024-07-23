import { createRouter, createWebHashHistory } from 'vue-router'
const routes = [
    {
        path: '/',
        name: 'style1',
        component: () => import('../pages/style1/style1.vue'),
    },
    {
        path: '/style2',
        name: 'style2',
        component: () => import('../pages/style2/style2.vue'),
    }
];

const router = createRouter({
    routes,
    history: createWebHashHistory(),
})


export default router