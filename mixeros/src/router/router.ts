import { createRouter, createWebHistory } from 'vue-router'

import ChannelEditorWindow from '../components/ChannelEditor/ChannelEditorWindow.vue'
import RoutingWindow from '../components/Routing/RoutingWindow.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: ChannelEditorWindow},
    { path: '/routing', component: RoutingWindow},
  ],
})

export default router
