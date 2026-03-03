<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';
  import TieredMenu from 'primevue/tieredmenu';

  const Filemenu = ref();
  const Applicationmenu = ref();

  const Fileitems = ref([
    {
      label: 'New Show'
    },
    {
      label: 'Load Show'
    },
    {
      seperator: true
    },
    {
      label: 'Save'
    },
    {
      label: 'Save As'
    },
    {
      seperator: true
    },
    {
      label: 'Export',
      items: [
        {
          label: 'Routing Definition'
        },
        {
          label: 'Show File'
        },
        {
          label: 'Settings'
        },
        {
          label: 'Presets'
        }
      ]
    },
    {
      label: 'Import',
      items: [
        {
          label: 'Routing Definition'
        },
        {
          label: 'Show File'
        },
        {
          label: 'Settings'
        },
        {
          label: 'Presets'
        }
      ]
    }
  ])

  const Applicationitems = ref([
    {
      label: 'Quit'
    },
    {
      label: 'Reboot'
    },
    {
      label: 'Shutdown'
    },
    {
      label: 'Lockout'
    }

  ])

  const Filetoggle = (event: any) => {
    Filemenu.value.toggle(event);
  };

  const Applicationtoggle = (event: any) => {
    Applicationmenu.value.toggle(event);
  };
</script>

<template>
  <div class="flex flex-row w-screen h-1 justify-between">
    <div class="gap-4 px-2 flex flex-row justify-center items-center">
      <button class="text-white hover:text-[#8b8b8b]" @click="Filetoggle">File</button>
      <TieredMenu ref="Filemenu" :model="Fileitems" popup >
        <template #item="{ item, props, hasSubmenu }">
          <a v-ripple class="flex align-items-center bg-[#282828] p-12" v-bind="props.action">
            <span class="ml-2 text-white">{{ item.label }}</span>
            <span v-if="item.shortcut" class="ml-auto border surface-border border-round surface-100 text-xs p-1">{{
              item.shortcut }}</span>
            <i v-if="hasSubmenu" class="pi pi-angle-right ml-auto"></i>
          </a>
        </template>
      </TieredMenu>
      
      <button class="text-white hover:text-[#8b8b8b]">Settings</button>

      <button class="text-white hover:text-[#8b8b8b]" @click="Applicationtoggle">Application</button>
      <TieredMenu ref="Applicationmenu" :model="Applicationitems" popup >
        <template #item="{ item, props, hasSubmenu }">
          <a v-ripple class="flex align-items-center bg-[#282828] p-12" v-bind="props.action">
            <span class="ml-2 text-white">{{ item.label }}</span>
            <span v-if="item.shortcut" class="ml-auto border surface-border border-round surface-100 text-xs p-1">{{
              item.shortcut }}</span>
            <i v-if="hasSubmenu" class="pi pi-angle-right ml-auto"></i>
          </a>
        </template>
      </TieredMenu>

      <button class="text-white hover:text-[#8b8b8b]">About</button>
    </div>
  </div>


</template>

