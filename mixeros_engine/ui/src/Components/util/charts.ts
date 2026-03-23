import { ChartDataset, Point } from 'chart.js';

export function toChartJsDatasets(data: { timestamp: number; cpu: { name: string; value: number }[] }[]): ChartDataset<'line', (number | Point | null)[]>[] {
  let cpuNames = [...new Set(data.flatMap(d => d.cpu.map(c => c.name)))];

  let datasets: ChartDataset<'line', (number | Point | null)[]>[] = cpuNames.map(name => ({
    label: name,
    data: data
      .filter(d => d.cpu.some(c => c.name === name))
      .map(d => ({
        x: d.timestamp,
        y: d.cpu.find(c => c.name === name)?.value ?? null,
      })),
  }));

  return datasets;
}
