<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { commands } from '../../bindings.ts'
import { path } from '@tauri-apps/api'
import { appDataDir } from '@tauri-apps/api/path'
import { useStore } from '../../store.ts'
import DownloadSettings from './components/DownloadSettings.vue'
import FmtSettings from './components/FmtSettings.vue'
import NetworkSettings from './components/NetworkSettings.vue'
import AssDanmakuSettings from './components/AssDanmakuSettings.vue'
import PluginSettings from './components/PluginSettings.vue'
import { NButton, NDialog, NModal, NTabPane, NTabs, useMessage } from 'naive-ui'

const store = useStore()

const message = useMessage()

const showing = defineModel<boolean>('showing', { required: true })

onMounted(async () => {
  store.config = await commands.getConfig()
})

watch(
  () => store.config,
  async () => {
    if (store.config === undefined) {
      return
    }

    const result = await commands.saveConfig(store.config)
    if (result.status === 'error') {
      console.error(result.error)
      return
    }
    message.success('保存配置成功')
  },
  { deep: true },
)

const currentTabName = ref<string>('download_settings')

async function showConfigInFileManager() {
  const configName = 'config.json'
  const configPath = await path.join(await appDataDir(), configName)
  const result = await commands.showPathInFileManager(configPath)
  if (result.status === 'error') {
    console.error(result.error)
  }
}
</script>

<template>
  <n-modal v-if="store.config !== undefined" v-model:show="showing">
    <n-dialog :showIcon="false" content-style="" @close="showing = false">
      <div class="flex flex-col gap-row-2">
        <n-tabs class="h-full settings-tabs" v-model:value="currentTabName" type="line" size="small" animated>
          <n-tab-pane name="download_settings" tab="下载内容">
            <DownloadSettings />
          </n-tab-pane>
          <n-tab-pane name="fmt_settings" tab="命名格式">
            <FmtSettings />
          </n-tab-pane>
          <n-tab-pane name="ass_danmaku_settings" tab="ass弹幕">
            <AssDanmakuSettings />
          </n-tab-pane>
          <n-tab-pane name="network_settings" tab="网络">
            <NetworkSettings />
          </n-tab-pane>
          <n-tab-pane name="plugin_settings" tab="插件">
            <PluginSettings />
          </n-tab-pane>
        </n-tabs>

        <n-button
          v-if="currentTabName !== 'plugin_settings'"
          class="ml-auto mt-2"
          size="small"
          @click="showConfigInFileManager">
          打开配置目录
        </n-button>
      </div>
    </n-dialog>
  </n-modal>
</template>
