<template>
  <div id="reports-chart" @click="goto_report"></div>
</template>

<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
const echarts = require('echarts');
import router from '@/bloom/kernel/router';


@Component
export default class ReportsChart extends Vue {
  // props
  @Prop({ type: Array, default: [] }) reports!: any[];


  // data
  chart: any | null = null;
  tooltip_position = 0;


  // lifecycle
  created() {
    Vue.nextTick(() => {
      this.render_chart();
    });
  }

  mounted() {
    window.addEventListener('resize', this.handle_resize);
  }

  beforeDestroy() {
    window.removeEventListener('resize', this.handle_resize);
  }


  // watch
  @Watch('reports')
  on_reports_change() {
    Vue.nextTick(() => {
      this.render_chart();
    });
  }

  // methods
    handle_resize() {
    if (this.chart) {
      this.chart.resize();
    }
  }

  goto_report() {
    const { scan_id } = this.$route.params;
    const report_id = this.reports[this.tooltip_position].id;
    router.push({ path: `/platform/phaser/${scan_id}/reports/${report_id}` });
  }

  render_chart() {
    const series = this.reports.map((point: any) => point.high_level_findings);
    const dates = this.reports.map((point) => {
      const d = new Date(point.created_at);
      return [d.getFullYear(), d.getMonth() + 1, d.getDate()].join('/');
    });
    this.chart = echarts.init(document.getElementById('reports-chart'));
    // specify chart configuration item and data
    const option = {
      series: [
        {
          areaStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              {
                color: 'rgb(255, 158, 68)',
                offset: 0,
              },
              {
                color: 'rgb(255, 70, 131)',
                offset: 1,
              },
            ]),
          },
          data: series,
          // name: 'views',
          smooth: true,
          type: 'line',
        },
      ],
      tooltip: {
        formatter: (params: any) => {
          this.tooltip_position = params[0].dataIndex;
        },
        position(pt: any) {
          return [pt[0], '10%'];
        },
        trigger: 'axis',
      },
      xAxis: {
        boundaryGap: false,
        data: dates,
        type: 'category',
      },
      yAxis: {
        type: 'value',
      },
    };
    // use configuration item and data specified to show chart
    this.chart.setOption(option);
    // this.chart.on('click', (x) => {
    //   const { scan_id } = this.$route.params;
    //   const report_id = this.reports[x.dataIndex].id;
    //   router.push({ path: `/platform/phaser/${scan_id}/reports/${report_id}` });
    // });
  }
}
</script>


<style scoped lang="scss">
#reports-chart {
  height: 250px;
  cursor: pointer !important;
}
</style>
