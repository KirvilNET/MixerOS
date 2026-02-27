<script setup lang="ts">
  import { ref } from "vue";
  import { RouterView } from "vue-router";

  import Loading from "./pages/Loading.vue";
  
  import MainApp from "./pages/mainApp.vue";

  const loaded = ref(true)
  const progress = ref(100)

</script>

<template>
  <template v-if="loaded == false">
    <Loading
      :progress="progress"
    />
  </template>
  <template v-if="loaded == true">
    <MainApp
      
    />
  </template>
</template>