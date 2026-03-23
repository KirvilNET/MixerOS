<script lang="ts" setup>
    import { ref } from "vue";

    import { Status, Type, NetworkPort } from '../schema/Network'

    import StartScreen from "./startScreen.vue";
    import { RouterView } from "vue-router";

    const isFileOpen = ref(true)
</script>

<template>
    <div class="bg-[#121212] min-h-screen overflow-hidden">
        <div class="flex flex-col gap-0">
            <div class="p-2 gap-2">
                <navbar />
            </div>
        </div>

        <template v-if="isFileOpen == false">
            <StartScreen />
        </template>

        <template v-else>
            <RouterView />
        </template>

        <div class="fixed bottom-0 py-0 ">
            <statusbar />
        </div>
    </div>
</template>