import { createRouter, createWebHistory} from "vue-router"

const Server = {
    template: "<div>{{ $route.params.id }}</div>"
}

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: () => import('../views/Home.vue')
        }
    ]
})

export default router