<script setup lang="ts">
  import { ref } from "vue";

  const version = ref('1.0.0')
</script>

<template>
  <div class="bg-[#121212] h-screen">
    <div class=" h-full flex justify-center items-center">
      <div class="bg-[#282828] rounded-lg h-132 w-5xl">
         <div class="grid grid-cols-4 grid-rows-4 gap-0">
          <div class="row-span-2">
            <div class=" flex justify-center items-center mt-8">
              <img src="/Icon.svg" class="size-36">
            </div>
          </div>
          <div class="row-span-2 col-start-2 row-start-1 border-r border-r-white">
            <div class="flex flex-col gap-2 mt-8 mr-8 text-right">
              <h1 class="text-white text-3xl"> MixerOS {{ version }}</h1>
              <p class="text-white">Changelog</p>
              <p class="text-white">Documentation</p>
              <p class="text-white">Support</p>
            </div>
          </div>
          <div class="col-span-2 row-span-4 col-start-3 row-start-1">
            <div class="flex justify-center items-center border-b border-white">
              <p class="text-white text-3xl">Recents</p>
            </div>
            <div class="flex flex-col">

            </div>
          </div>
          <div class="col-span-2 row-span-2 col-start-1 row-start-3 border-r border-r-white h-full">
            <div class="flex flex-col min-h-full gap-6">
              <button class="hover:bg-[#3f3f3f]">
                <div class="flex flex-row gap-4">
                  <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24">
                    <path fill="#fff"
                      d="M4 20q-.825 0-1.412-.587T2 18V6q0-.825.588-1.412T4 4h6l2 2h8q.825 0 1.413.588T22 8v10q0 .825-.587 1.413T20 20z" />
                  </svg>
                  <div class="flex flex-col gap-1 text-left">
                    <p class="text-white text-2xl">Open Show</p>
                    <p class="text-white text-md">From File</p>
                  </div>
                </div>
              </button>
              <button class="hover:bg-[#3f3f3f]">
                <div class="flex flex-row gap-4">
                  <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24">
                    <path fill="#fff"
                      d="M13 9V3.5L18.5 9M6 2c-1.11 0-2 .89-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6z" />
                  </svg>
                  <div class="flex flex-col gap-1 text-left">
                    <p class="text-white text-2xl">New Show</p>
                    <p class="text-white text-md">From Built-in Blank Template</p>
                  </div>
                </div>
              </button>
              <button class="hover:bg-[#3f3f3f]">
                <div class="flex flex-row gap-4">
                  <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24">
                    <path fill="none" stroke="#fff" stroke-linecap="round" stroke-width="1.5"
                      d="M4.513 19.487c2.512 2.392 5.503 1.435 6.7.466c.618-.501.897-.825 1.136-1.065c.837-.777.784-1.555.24-2.177c-.219-.249-1.616-1.591-2.956-2.967c-.694-.694-1.172-1.184-1.582-1.58c-.547-.546-1.026-1.172-1.744-1.154c-.658 0-1.136.58-1.735 1.179c-.688.688-1.196 1.555-1.375 2.333c-.539 2.273.299 3.888 1.316 4.965Zm0 0L2 21.999M19.487 4.515c-2.513-2.394-5.494-1.42-6.69-.45c-.62.502-.898.826-1.138 1.066c-.837.778-.784 1.556-.239 2.178c.078.09.31.32.635.644m7.432-3.438c1.017 1.077 1.866 2.71 1.327 4.985c-.18.778-.688 1.645-1.376 2.334c-.598.598-1.077 1.179-1.735 1.179c-.718.018-1.09-.502-1.639-1.048m3.423-7.45L22 2m-5.936 9.964c-.41-.395-.994-.993-1.688-1.687c-.858-.882-1.74-1.75-2.321-2.325m4.009 4.012l-1.562 1.525m-3.99-3.984l1.543-1.553" />
                  </svg>
                  <div class="flex flex-col gap-1 text-left">
                    <p class="text-white text-2xl">Connect To Show</p>
                    <p class="text-white text-md">On Local Network</p>
                  </div>
                </div>
              </button>
            </div>
          </div>
      </div>
      </div>
    </div>
  </div>
</template>