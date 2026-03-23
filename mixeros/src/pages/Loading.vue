<script setup lang="ts">
    import { ref, onMounted, watch } from "vue";
    import router from "../router/router";
    
    const props = defineProps<{
        progress: number,
    }>()

    watch(() => props.progress, (x) => {
        if (x == 100) router.push('/app')
    })

</script>

<template>
    <div class="flex flex-col justify-center items-center bg-[#121212] h-screen">
        <div class="flex flex-col items-center gap-4">
            <div class="border-b-4">
                <img class="size-48" src="/Icon.svg">
            </div>
            <div class="relative flex items-center w-80 bg-[#3f3f3f] h-5 rounded-full">
                <div class="bg-linear-to-r from-[#ff005c] to-[#ff637d] h-5 rounded-full"
                    :style="`width: ${progress}%`"></div>
                <span class="absolute inset-0 flex items-center justify-center text-xs font-semibold text-white">
                    {{ props.progress }}%
                </span>
            </div>
            <p class="text-white">test</p>
        </div>
    </div>
</template>