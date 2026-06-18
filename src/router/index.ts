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
  },{
    path:"/Login",
    name:"Login",
    component:()=>import("@/view/Login/index.vue"),
  },{
    path:"/errors",
    name:"errors",
    component:()=>import("@/view/Errors/index.vue"),
  }],
});
export default router;