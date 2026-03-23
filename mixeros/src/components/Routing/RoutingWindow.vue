<script setup lang="ts">
    import { ref, onMounted, onUnmounted } from 'vue';

    import routerGUI from './routerGUI.vue';
    import DeviceTable from './deviceTable.vue';

    const page = ref(true)

    function ToDeviceTable() {
        page.value = true
    }

    function TorouterGUI() {
        page.value = false
    }
</script>

<template>
    <div class="flex flex-row mx-4 gap-0 border-b border-[#ff005c]">
        <button @click="ToDeviceTable" class="text-white text-3xl px-4 focus:border-b hover:border-b border-[#ff005c] hover:bg-[#3f3f3f]">Device Table</button>
        <button @click="TorouterGUI" class="text-white text-3xl px-4 focus:border-b hover:border-b border-[#ff005c] hover:bg-[#3f3f3f]">Routing</button>
    </div>
    <div class="px-4 mt-4">
        <template v-if="page == true">
            <DeviceTable
                :devices="[]"
            />
        </template>
        <template v-if="page == false">
            <routerGUI />
        </template>
    </div>
</template>