1<script setup lang="ts">
  import { ref, onMounted, onUnmounted } from 'vue';

  const currentTime = ref({ hour: 0, min: 0, second: 0, });
  let intervalId: number | null = null;

  const getNow = () => {
    const today = new Date();

    currentTime.value.hour = today.getHours()
    currentTime.value.min = today.getMinutes()
    currentTime.value.second = today.getSeconds()
  };

  onMounted(() => {
    getNow();
    intervalId = setInterval(getNow, 1000);
  });

  onUnmounted(() => {
    if (intervalId !== null) clearInterval(intervalId!);
  });
</script>

<template>
  <div class="flex flex-row w-screen h-12 bg-[#282828] border-t-[#ff005c] justify-between">
    <div class="gap-4 px-2 flex flex-row justify-center items-center">
      <p class="text-white font-size-22px">Current Time: <span class="text-white font-size-12px">{{ currentTime.hour }}:{{ currentTime.min }}:{{ currentTime.second }}</span></p>
      
    </div>
    <div class="gap-2 px-8 flex flex-row justify-center items-center">
      <div class="gap-1 hover:bg-[#3f3f3f] px-2 h-full flex justify-center items-center">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="#fff"
            d="M10 2c-1.11 0-2 .89-2 2v3c0 1.11.89 2 2 2h1v2H2v2h4v2H5c-1.11 0-2 .89-2 2v3c0 1.11.89 2 2 2h4c1.11 0 2-.89 2-2v-3c0-1.11-.89-2-2-2H8v-2h8v2h-1c-1.11 0-2 .89-2 2v3c0 1.11.89 2 2 2h4c1.11 0 2-.89 2-2v-3c0-1.11-.89-2-2-2h-1v-2h4v-2h-9V9h1c1.11 0 2-.89 2-2V4c0-1.11-.89-2-2-2zm0 2h4v3h-4zM5 17h4v3H5zm10 0h4v3h-4z" />
        </svg>
        <div class="h-3 w-3 rounded-4xl bg-green-600"></div>
      </div>
      <div class="gap-1 hover:bg-[#3f3f3f] px-2 h-full flex justify-center items-center">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="#fff" d="M7 4v2h3v2H7l-2 2v3H3v-3H1v8h2v-3h2v3h3l2 2h8v-4h2v3h3V9h-3v3h-2V8h-6V6h3V4z" />
        </svg>
        <div class="h-3 w-3 rounded-4xl bg-green-600"></div>
      </div>
      <div class="gap-1 hover:bg-[#3f3f3f] px-2 h-full flex justify-center items-center">    
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
          <path fill="#fff"
            d="M14 9h-4a1 1 0 0 0-1 1v4a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1m-1 4h-2v-2h2Zm8 0a1 1 0 0 0 0-2h-2V9h2a1 1 0 0 0 0-2h-2.18A3 3 0 0 0 17 5.18V3a1 1 0 0 0-2 0v2h-2V3a1 1 0 0 0-2 0v2H9V3a1 1 0 0 0-2 0v2.18A3 3 0 0 0 5.18 7H3a1 1 0 0 0 0 2h2v2H3a1 1 0 0 0 0 2h2v2H3a1 1 0 0 0 0 2h2.18A3 3 0 0 0 7 18.82V21a1 1 0 0 0 2 0v-2h2v2a1 1 0 0 0 2 0v-2h2v2a1 1 0 0 0 2 0v-2.18A3 3 0 0 0 18.82 17H21a1 1 0 0 0 0-2h-2v-2Zm-4 3a1 1 0 0 1-1 1H8a1 1 0 0 1-1-1V8a1 1 0 0 1 1-1h8a1 1 0 0 1 1 1Z" />
        </svg>
        <div class="h-3 w-3 rounded-4xl bg-green-600"></div>
      </div>
    </div>
  </div>
</template>