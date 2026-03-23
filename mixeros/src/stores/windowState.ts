import { ref } from 'vue';
import { defineStore } from 'pinia';

import { display, displayState } from '../schema/window';
import { invoke } from '@tauri-apps/api/core';

export const useDisplayStateStore = defineStore('displayState', () => {
    const windows = ref<display[]>([])
    
    function addDisplay(display: display) {
        windows.value.push(display)
    }

    return { windows, addDisplay }
});