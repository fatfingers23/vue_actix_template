import { createRouter, createWebHistory } from 'vue-router';
import Todo from '../views/TodoView.vue';
import NotFound from '../views/NotFound.vue';
import About from '../views/AboutView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'todo',
      component: Todo
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: About
    },
    { path: '/:pathMatch(.*)*', name: 'NotFound', component: NotFound },
  ]
});

export default router;
