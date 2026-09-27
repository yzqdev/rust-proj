<script setup lang="tsx">
import { computed, defineComponent, onMounted, onUnmounted, ref, useTemplateRef, type PropType } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { NButton, NCollapseTransition, NInputNumber, NSwitch, NTag, useMessage, NEmpty, NIcon } from 'naive-ui'
import { commands, events, PluginInfo } from '../../../bindings.ts'
import { PhPlusCircle } from '@phosphor-icons/vue'
import { path } from '@tauri-apps/api'
import { appDataDir } from '@tauri-apps/api/path'
import { getCurrentWindow } from '@tauri-apps/api/window'

const message = useMessage()
const allowedPluginExtensions = new Set(['.dll', '.so', '.dylib'])

const pluginInfos = ref<Map<string, PluginInfo>>(new Map())

const sortedPluginInfos = computed<PluginInfo[]>(() =>
  Array.from(pluginInfos.value.values()).sort((a, b) => b.priority - a.priority),
)
const expandedPluginIds = ref<Set<string>>(new Set())
const dropZoneRef = useTemplateRef('dropZoneRef')
const isDragOverDropZone = ref<boolean>(false)

onMounted(async () => {
  const infos = await commands.getPluginInfos()

  const pluginMap = new Map<string, PluginInfo>()

  for (const info of infos) {
    pluginMap.set(info.path, info)
  }

  pluginInfos.value = pluginMap
})

let unListenPluginEvent: (() => void) | undefined
let unListenDragDropEvent: (() => void) | undefined
onMounted(() => {
  events.pluginEvent
    .listen(({ payload: { event, data } }) => {
      if (event === 'Loaded') {
        const info = data.plugin_info

        pluginInfos.value.set(info.path, info)
      } else if (event === 'Update') {
        const info = data.plugin_info

        const pluginInfo = pluginInfos.value.get(info.path)
        if (pluginInfo !== undefined) {
          Object.assign(pluginInfo, info)
        }
      } else if (event === 'Uninstall') {
        const pluginPath = data.plugin_path

        pluginInfos.value.delete(pluginPath)
        expandedPluginIds.value.delete(pluginPath)
      }
    })
    .then((unListenFn) => {
      unListenPluginEvent = unListenFn
    })
  getCurrentWindow()
    .onDragDropEvent(({ payload }) => {
      if (payload.type === 'leave') {
        isDragOverDropZone.value = false
        return
      }

      if (payload.type === 'enter' || payload.type === 'over') {
        isDragOverDropZone.value = isPointInDropZone(payload.position.x, payload.position.y)
        return
      }

      const droppingOnDropZone = isPointInDropZone(payload.position.x, payload.position.y)
      isDragOverDropZone.value = false
      if (!droppingOnDropZone) {
        return
      }

      addPluginsByPaths(payload.paths)
    })
    .then((unListenFn) => {
      unListenDragDropEvent = unListenFn
    })
})

onUnmounted(() => {
  unListenPluginEvent?.()
  unListenDragDropEvent?.()
})

function getFileName(pluginPath: string): string {
  const normalizedPath = pluginPath.replace(/\\/g, '/')
  const parts = normalizedPath.split('/')
  const filename = parts[parts.length - 1]
  return filename === '' ? pluginPath : filename
}

function getFileExtension(pluginPath: string): string {
  const filename = getFileName(pluginPath)
  const dotIndex = filename.lastIndexOf('.')
  if (dotIndex <= 0) {
    return ''
  }
  return filename.slice(dotIndex).toLowerCase()
}

function isPluginDynamicLibrary(pluginPath: string): boolean {
  return allowedPluginExtensions.has(getFileExtension(pluginPath))
}

function isPointInDropZone(rawX: number, rawY: number): boolean {
  const dropZone = dropZoneRef.value
  if (dropZone === null) {
    return false
  }

  const rect = dropZone.getBoundingClientRect()
  const dpr = window.devicePixelRatio || 1

  const candidates: Array<[number, number]> = [[rawX, rawY]]
  if (dpr !== 1) {
    candidates.push([rawX / dpr, rawY / dpr])
  }

  return candidates.some(([x, y]) => x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom)
}

async function addPluginByPath(pluginPath: string) {
  const result = await commands.addPlugin(pluginPath)
  const filename = getFileName(pluginPath)

  if (result.status === 'error') {
    console.error(result.error)
    return
  }

  message.success(`添加插件成功: ${filename}`)
}

async function addPluginsByPaths(pluginPaths: string[]) {
  for (const pluginPath of pluginPaths) {
    const filename = getFileName(pluginPath)
    if (!isPluginDynamicLibrary(pluginPath)) {
      message.warning(`跳过非插件文件: ${filename}`)
      continue
    }

    await addPluginByPath(pluginPath)
  }
}

async function addPlugin() {
  const selectedPath = await open({
    directory: false,
    multiple: false,
    filters: [{ name: '', extensions: ['dll', 'so', 'dylib'] }],
  })
  if (selectedPath === null) {
    return
  }

  await addPluginByPath(selectedPath)
}

async function showPluginConfigInFileManager() {
  const configPath = await path.join(await appDataDir(), 'plugin.json')
  const result = await commands.showPathInFileManager(configPath)
  if (result.status === 'error') {
    console.error(result.error)
  }
}

const PluginCard = defineComponent({
  name: 'PluginCard',
  props: {
    pluginInfo: {
      type: Object as PropType<PluginInfo>,
      required: true,
    },
  },
  setup(props) {
    function toggleAdvanced(pluginPath: string) {
      if (expandedPluginIds.value.has(pluginPath)) {
        expandedPluginIds.value.delete(pluginPath)
      } else {
        expandedPluginIds.value.add(pluginPath)
      }
    }

    function isAdvancedShown(pluginPath: string): boolean {
      return expandedPluginIds.value.has(pluginPath)
    }

    return () => {
      async function uninstallPlugin(pluginPath: string) {
        const result = await commands.uninstallPlugin(pluginPath)
        if (result.status === 'error') {
          console.error(result.error)
          return
        }

        expandedPluginIds.value.delete(pluginPath)
        message.success('卸载插件成功')
      }

      async function updateEnabled(pluginPath: string, enabled: boolean) {
        const pluginInfo = pluginInfos.value.get(pluginPath)
        if (pluginInfo === undefined) {
          return
        }

        const prevEnabled = pluginInfo.enabled

        pluginInfo.enabled = enabled

        const result = await commands.setPluginEnabled(pluginPath, enabled)
        if (result.status === 'error') {
          console.error(result.error)
          pluginInfo.enabled = prevEnabled
        }
      }

      async function updatePriority(pluginPath: string, priority: number | null) {
        if (priority === null) {
          return
        }

        const pluginInfo = pluginInfos.value.get(pluginPath)
        if (pluginInfo === undefined) {
          return
        }

        const prevPriority = pluginInfo.priority

        const result = await commands.setPluginPriority(pluginPath, priority)
        if (result.status === 'error') {
          console.error(result.error)
          pluginInfo.priority = prevPriority
        }
      }

      type TagType = 'error' | 'default' | 'primary' | 'info' | 'success' | 'warning'

      function getRuntimeStatusMeta(status: PluginInfo['runtime_status']): { text: string; type: TagType } {
        switch (status) {
          case 'Loaded':
            return { text: '已加载', type: 'success' }
          case 'Disabled':
            return { text: '未启用', type: 'warning' }
          case 'LoadFailed':
            return { text: '加载失败', type: 'error' }
          case 'Unknown':
            return { text: '待加载', type: 'info' }
        }
      }

      function getFailurePolicyMeta(policy: PluginInfo['descriptor']['failure_policy']): {
        text: string
        type: TagType
      } {
        if (policy === 'FailClosed') {
          return { text: 'FailClosed', type: 'warning' }
        }
        return { text: 'FailOpen', type: 'info' }
      }

      const runtimeStatus = getRuntimeStatusMeta(props.pluginInfo.runtime_status)
      const failurePolicy = getFailurePolicyMeta(props.pluginInfo.descriptor.failure_policy)
      const advancedShowing = isAdvancedShown(props.pluginInfo.path)

      return (
        <div class="rounded border-1 border-solid border-gray-3 px-3 py-2 flex flex-col gap-1">
          <div class="flex items-center flex-wrap gap-2">
            <span class="font-bold text-sm">{props.pluginInfo.descriptor.name}</span>
            <NTag size="small">v{props.pluginInfo.descriptor.version}</NTag>
            <NTag size="small" type={runtimeStatus.type}>
              {runtimeStatus.text}
            </NTag>
          </div>

          <div class="text-xs text-gray-6 break-words">{props.pluginInfo.descriptor.description}</div>

          <div class="flex flex-col">
            <NCollapseTransition show={advancedShowing}>
              <div class="flex flex-col gap-1">
                <div class="text-xs break-all">
                  <span class="font-bold">SDK API:</span>
                  <span class="ml-1">{props.pluginInfo.descriptor.sdk_api_version}</span>
                </div>

                <div class="text-xs break-all">
                  <span class="font-bold">ID:</span>
                  <span class="ml-1">{props.pluginInfo.descriptor.id}</span>
                </div>

                <div class="text-xs break-all">
                  <span class="font-bold">路径:</span>
                  <span class="ml-1">{props.pluginInfo.path}</span>
                </div>

                <div class="flex items-center gap-1 flex-wrap">
                  <span class="text-xs font-bold">Hooks:</span>
                  {props.pluginInfo.descriptor.hooks.map((hookPoint) => (
                    <NTag key={hookPoint} size="small">
                      {hookPoint}
                    </NTag>
                  ))}
                </div>

                <div class="flex items-center gap-1">
                  <span class="text-xs font-bold">失败策略:</span>
                  <NTag size="small" type={failurePolicy.type}>
                    {failurePolicy.text}
                  </NTag>
                </div>
              </div>
            </NCollapseTransition>

            <div class="flex gap-4">
              <div class="flex items-center gap-2">
                <span class="text-sm">启用</span>
                <NSwitch
                  size="small"
                  value={props.pluginInfo.enabled}
                  onUpdate:value={(value: boolean) => updateEnabled(props.pluginInfo.path, value)}
                />
              </div>

              <div class="flex items-center gap-2">
                <span class="text-sm">优先级</span>
                <NInputNumber
                  class="w-26"
                  size="small"
                  value={props.pluginInfo.priority}
                  onUpdate:value={(value: number | null) => updatePriority(props.pluginInfo.path, value)}
                  parse={(x: string) => Number(x)}
                />
              </div>
            </div>

            <div class="flex w-full items-center gap-2">
              <NButton size="small" onClick={() => toggleAdvanced(props.pluginInfo.path)}>
                {advancedShowing ? '隐藏高级信息' : '显示高级信息'}
              </NButton>

              <NButton
                class="ml-auto"
                size="small"
                type="error"
                ghost
                onClick={() => uninstallPlugin(props.pluginInfo.path)}>
                卸载
              </NButton>
            </div>
          </div>
        </div>
      )
    }
  },
})
</script>

<template>
  <div class="flex flex-col gap-row-2">
    <n-empty v-if="sortedPluginInfos.length === 0" description="暂无插件，点击 添加插件 按钮导入插件" />
    <div v-else class="flex flex-col gap-2 max-h-60vh overflow-auto overflow-x-hidden pr-1">
      <PluginCard v-for="pluginInfo in sortedPluginInfos" :key="pluginInfo.path" :plugin-info="pluginInfo" />
    </div>

    <div class="mt-2 flex items-center gap-2">
      <div
        ref="dropZoneRef"
        class="flex items-center gap-2 rounded-md border border-dashed px-2 py-1 transition-colors"
        :class="isDragOverDropZone ? 'border-sky-4 bg-sky-1/70' : 'border-gray-3 bg-transparent'">
        <n-button size="small" type="primary" @click="addPlugin">
          <template #icon>
            <n-icon size="20">
              <PhPlusCircle />
            </n-icon>
          </template>
          添加插件
        </n-button>
        <span class="text-xs text-gray-6">也可以拖入此处导入</span>
      </div>

      <n-button class="ml-auto" size="small" @click="showPluginConfigInFileManager">打开配置目录</n-button>
    </div>
  </div>
</template>
