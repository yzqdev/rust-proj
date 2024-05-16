<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { createDir, BaseDirectory ,exists} from '@tauri-apps/api/fs';
// Create the `$APPDATA/users` directory

const greetMsg = ref("");
const name = ref("");
import { getTauriVersion } from '@tauri-apps/api/app';
const v=ref('')
 
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  v.value = await getTauriVersion();
  await createDir('users', { dir: BaseDirectory.Picture, recursive: true });
 const res= await exists('users/avatar.png', { dir: BaseDirectory.Picture });
 console.log(res)
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet </button>

    <div>{{v }}</div>
  </form>
    <img src="https://upload-bbs.miyoushe.com/upload/2024/05/16/352753404/d1b1b32d9f281d7cb91f5f0bbc3f9098_1039289690596327266.png?x-oss-process=image/resize,s_600/quality,q_80/auto-orient,0/interlace,1/format,png"/>
  <p>{{ greetMsg }}</p>
</template>
