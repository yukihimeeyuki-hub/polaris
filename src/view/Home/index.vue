<script setup lang="ts">
import {
  BaseDirectory,
    create,
    writeTextFile,
  readTextFile,
  // remove,
  exists,
} from '@tauri-apps/plugin-fs';
import {Button} from "@/components/ui/button";

const  createFile=async ()=>{
  const isExists=await exists("config.json",{baseDir:BaseDirectory.AppLocalData});
  if(!isExists){
    await create("config.json",{baseDir:BaseDirectory.AppLocalData});
    await writeTextFile("config.json",'{"name":"tauri"}',{baseDir:BaseDirectory.AppLocalData});
  }else{
    const data=await readTextFile("config.json",{baseDir:BaseDirectory.AppLocalData});
    console.log(data);
  }
}

</script>

<template>
<div class="home">

<Button @click="createFile">
  读取文件
</Button></div>
</template>

<style scoped lang="scss">

</style>