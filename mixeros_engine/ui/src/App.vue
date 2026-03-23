<script setup lang="ts">
  import sidebar from "./Components/nav/sidebar.vue"
  import statCard from "./Components/dashboard/cpuStatCard.vue";

  let cpu = fetch("/metrics/cpu")

  
</script>

<template>
  <div className="grid grid-cols-7 grid-rows-5 gap-4 bg-[#121212] max-w-screen">
    <div className="row-span-5">
      <sidebar />
    </div>
    <div className="col-span-6 row-span-5 bg-[#121212] max-w-full sm:mx-auto sm:px-6 lg:px-8">
      <div class="max-w-full mx-4 py-6 sm:mx-auto sm:px-6 lg:px-8">
        <div class="flex">
          <statCard
            :name="'CPU'"
            :data="data"
          />
        </div>
    </div>
      </div>

  </div>
    


</template>
