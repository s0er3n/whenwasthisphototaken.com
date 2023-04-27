
import { Component, createEffect, createSignal, onMount } from 'solid-js';
import Chart from 'chart.js/auto';
import annotationPlugin from 'chartjs-plugin-annotation';
import { result, state } from './Lobby';
Chart.register(annotationPlugin);
Chart.defaults.plugins.legend.display = false

const START = 1900;
const END = 2024;


let [data, setData] = createSignal([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])

function median(numbers: number[]): Number {
  let sorted: number[] = [];
  for (let i = 0; i <= END - START; i++) {
    for (let j = 0; j < numbers[i]; j++) {
      sorted.push(START + i)
    }
  }
  console.log(sorted)
  const middle = Math.floor(sorted.length / 2);

  if (sorted.length % 2 === 0) {
    return (sorted[middle - 1] + sorted[middle]) / 2;
  }

  return sorted[middle];
}
async function initChart(chartDiv: HTMLCanvasElement): Promise<Chart> {

  return new Chart(
    chartDiv,
    {
      type: 'bar',
      data: {
        labels: Array.apply(null, { length: END - START }).map(Number.call, Number).map((n: number) => n + START),
        datasets: [
          {
            data: data()
          }
        ]
      }
    }
  )
}
const ChartComponent: Component = () => {
  let chartDiv: HTMLCanvasElement;

  let chart: Chart;


  onMount(async () => {
    chart = await initChart(chartDiv)
    createEffect(() => {
      // let m = median(data()) - START
      // console.log(m)

      chart.data.datasets = [{
        labels: Array.apply(null, { length: 124 }).map(Number.call, Number).map((n: number) => n + 1900),
        data: data(),
      }]

      if (state() === "afterImage") {
        chart.options = {
          plugins: {
            annotation: {
              annotations: {
                line1: {
                  type: 'line',
                  xMin: result() - START,
                  xMax: result() - START,
                  borderColor: 'rgb(255, 99, 132)',
                  borderWidth: 6,
                }
              }
            }
          }
        }
      } else {
        chart.options = undefined
      }

      chart.update("default")
    });
  })
  return (<>
    <canvas height="20px" ref={(e) => chartDiv = e} />

  </>
  );
};

export default ChartComponent;
export { setData }
