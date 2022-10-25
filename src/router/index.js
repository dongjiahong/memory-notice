import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import TaskDigest from '../components/TaskDigest.vue'
import TaskDetail from '../components/TaskDetail.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
      children: [
        {
          path: '/task/:task_type',
          name: 'task_digest',
          component: TaskDigest,
          children: [
            {
              path: '/task/:task_type/:id',
              name: 'task_detail',
              component: TaskDetail
            }

          ]
        }
      ]
    },
  ]
})

export default router
