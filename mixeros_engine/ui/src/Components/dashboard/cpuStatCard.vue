<script setup lang="ts">
  import { Line } from 'vue-chartjs'
  import { Chart as ChartJS, Title, Tooltip, Legend, PointElement, CategoryScale, LinearScale, LineElement, Colors, ChartData} from 'chart.js'
  import { watch } from 'vue';
  import { toChartJsDatasets } from '../util/charts';
  

  const props = defineProps<{
    name: string,
    unit: string,
    data: Array<{timestamp: number, cpu: Array<{name: string, value: number}> }>
  }>()

  const name: string = "CPU";

  ChartJS.register(Title, Tooltip, Legend, PointElement, LineElement, CategoryScale, LinearScale, Colors)

  let chartData: ChartData<'line'> = {
    datasets: [],
  }

  let chartOptions = {
    responsive: true,
    color: "white"
  }

  watch(props.data, (newValue) => {
    let datasets = toChartJsDatasets(newValue);

    for (let i: number = 0; i < datasets.length; i++) {
      chartData.datasets.push(datasets[i]);
    }
    
  });


  
</script>

<template>
  <div class="flex flex-col gap-4 rounded-lg bg-[#282828] p-6">
    <div class="items-left">
      <h1 class="text-white ">{{ name }}</h1>
      <p class="text-gray-400"></p>
    </div>
    <div class="w-96">
      <Line 
        :id="'chart-'+ name"
        :options="chartOptions"
        :data="chartData"
      />
    </div>
  </div>
</template>