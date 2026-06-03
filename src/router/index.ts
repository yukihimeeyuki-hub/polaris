import {createRouter, createWebHistory} from "vue-router";


const router=createRouter({
  history: createWebHistory(),
  routes:[{
    path:"/",
    name:"Home",
    component:()=>import("@/view/Home/index.vue"),
  },{
    path:"/PromptBox",
    name:"PromptBox",
    component:()=>import("@/view/PromptBox.vue"),
  }],
});
export default router;