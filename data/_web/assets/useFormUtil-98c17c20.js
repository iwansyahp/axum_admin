import{h as e}from"./index.js";const t=()=>({formReset:a,formValidate:s}),a=e=>{e&&e.resetFields()},s=async t=>{let a=!1;if(!t)return a;const{i18n:s}=e(),{t:i}=s.global;return await t.validate((e=>{e?a=!0:ElementPlus.ElMessage.warning(i("commonTip.submitInvalid"))})),a};export{t as u};