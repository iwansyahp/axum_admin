import{z as e}from"./index.js";var t=(e=>(e.reload="reload",e.current="current",e.left="left",e.right="right",e.others="others",e.all="all",e))(t||{});const r=()=>{const t=VueRouter.useRouter(),r=VueRouter.useRoute(),u=e(),o=Vue.computed((()=>u.getTabList)),a=()=>o.value.findIndex((e=>e.fullPath===r.fullPath)),l=o.value[a()];return{tagList:o,goto:e=>{t.push({...e})},findCurrentRouteIndex:a,currentRoute:l}};export{t as E,r as u};