<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';
  import { useRouter, useRoute } from 'vue-router'

  import TieredMenu from 'primevue/tieredmenu';

  const menu = ref();
  const currentWindow = ref('Main Display')
  const router = useRouter()

  const Displayitems = ref([
    {
      label: 'Main Display',
      shortcut: 'Ctl+1',
      command: () => {
        router.push('/')
        currentWindow.value = 'Main Display'
      }
    },
    {
      label: 'Routing',
      shortcut: 'Ctl+2',
      command: () => {
        router.push('/routing')
        currentWindow.value = 'Routing'
      }
    },
    {
      label: 'Communication',
      shortcut: 'Ctl+3',
      command: () => {
        router.push('/comms')
        currentWindow.value = 'Communication'
      }
    },
    {
      label: 'Meters',
      shortcut: 'Ctl+4',
      command: () => {
        router.push('/meters')
        currentWindow.value = 'Meters'
      }
    },
    {
      label: 'FX',
      shortcut: 'Ctl+5',
      command: () => {
        router.push('/fx')
        currentWindow.value = 'FX'
      }
    },
    {
      label: 'Settings',
      shortcut: 'Ctl+6',
      command: () => {
        router.push('/settings')
        currentWindow.value = 'Settings'
      }
    },
    {
      label: 'System',
      shortcut: 'Ctl+7',
      command: () => {
        router.push('/system')
        currentWindow.value = 'System'
      }
    }
  ])

  const Displaytoggle = (event: any) => {
    menu.value.toggle(event);
  };

</script>

<template>
  <nav class="relative bg-[#282828] rounded ">
    <div class="mx-auto w-full px-2 ">
      <div class="relative flex flex-row w-full h-16 items-center justify-between gap-8">
        <div class="h-full flex flex-row items-center gap-4">
          <div class="justify-center items-center">
            <img src="/Icon.svg" class="size-12">
          </div>
          <div class="items-center h-16 w-36 bg-[#3f3f3f]">
            <button @click="Displaytoggle" class="flex flex-row items-center mt-4 justify-center gap-2">
              <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 16 16" class="text-[#ffffff]">
                <path fill="currentColor"
                  d="M1 2.75A.75.75 0 0 1 1.75 2h12.5a.75.75 0 0 1 0 1.5H1.75A.75.75 0 0 1 1 2.75m0 5A.75.75 0 0 1 1.75 7h12.5a.75.75 0 0 1 0 1.5H1.75A.75.75 0 0 1 1 7.75M1.75 12h12.5a.75.75 0 0 1 0 1.5H1.75a.75.75 0 0 1 0-1.5" />
              </svg>
              <p class="text-l text-white">{{ currentWindow }}</p>
            </button>
            <TieredMenu class="mt-4" ref="menu" :model="Displayitems" popup>
              <template #item="{ item, props, hasSubmenu }" >
                <a v-ripple class="flex align-items-center bg-[#282828] p-12" v-bind="props.action">
                  <span class="ml-2 text-white">{{ item.label }}</span>
                  <span v-if="item.shortcut" class="ml-auto text-white surface-100 text-xs p-1">{{
                    item.shortcut }}</span>
                  <i v-if="hasSubmenu" class="pi pi-angle-right ml-auto"></i>
                </a>
              </template>
            </TieredMenu>
          </div>
        </div>
        <div class="h-full flex flex-row items-center gap-4 shrink-0">
          <div class="items-center mt-8 h-16 w-48 ">
            <h1 class="text-4xl text-white">00:00:00:00</h1>
          </div>
        </div>
        <div class="flex flex-col items-left mr-12 text-white">
          <p>
            Active Show: <span class="text-gray"></span>
          </p>
          <p>
            Scene: <span class="text-gray"></span>
          </p>
        </div>
      </div>    
    </div>
  </nav>
</template>
