<template>
  <div>
    <v-layout justify-center >
        <v-progress-circular
        v-if="is_loading"
        indeterminate
        color="primary"
        ></v-progress-circular>
      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
      </v-flex>
    </v-layout>
    <v-layout v-if="report" row wrap>


      <v-flex xs12>
        <v-card class="elevation-0">
          <v-card-title class="pb-0">
            <h2 class="headline grey--text text--darken-2">
              Reports
            </h2>
          </v-card-title>
          <v-card-text class="pt-0">
            <blm-phaser-chart-reports :reports="reports" />
          </v-card-text>
        </v-card>
        <v-divider></v-divider>
      </v-flex>

      <v-flex xs12 class="mt-5 text-xs-center" v-if="report.status === 'SCANNING'">
        <v-progress-circular
        indeterminate
        color="primary"
        ></v-progress-circular>
        <div class="mt-5">
          <p>Scan in progress</p>
        </div>
      </v-flex>


      <v-flex xs12 v-if="report.status !== 'SCANNING'">
        <v-card class="elevation-0">
          <v-card-title>
            <h2 class="headline grey--text text--darken-2">
              Overview
            </h2>
          </v-card-title>
          <v-card-text class="pt-0">
            <v-container>
            <v-layout row wrap>
              <v-flex xs6 sm3 class="text-xs-center">
                <p class="title red--text">{{ report.high_level_findings }}</p>
                <p class="title red--text">High</p>
              </v-flex>
              <v-flex xs6 sm3 class="text-xs-center">
                <p class="title orange--text">{{ report.medium_level_findings }}</p>
                <p class="title orange--text">Medium</p>
              </v-flex>
              <v-flex xs6 sm3 class="text-xs-center">
                <p class="title grey--text">{{ report.low_level_findings }}</p>
                <p class="title grey--text">Low</p>
              </v-flex>
              <v-flex xs6 sm3 class="text-xs-center">
                <p class="title blue--text">{{ report.information_findings }}</p>
                <p class="title blue--text">Information</p>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Started</v-subheader>
                <span class="ml-3">{{ report.started_at | date }}</span>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Completed</v-subheader>
                <span class="ml-3">{{ report.completed_at | date }}</span>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Duration</v-subheader>
                <span class="ml-3">{{ report.duration | duration }}</span>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Target</v-subheader>
                <span class="ml-3">{{ report.target }}</span>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Profile</v-subheader>
                <span class="ml-3">{{ report.profile }}</span>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Trigger</v-subheader>
                <span class="ml-3">{{ report.trigger }}</span>
              </v-flex>

              <v-flex xs6 sm3>
                <v-subheader>Status</v-subheader>
                <span class="ml-3">{{ report.status }}</span>
              </v-flex>

            </v-layout>
            </v-container>
          </v-card-text>
        </v-card>
        <v-divider></v-divider>
      </v-flex>


      <v-flex xs12 v-if="report.status !== 'SCANNING'">
        <v-card class="elevation-0">
          <v-card-title>
            <h2 class="headline grey--text text--darken-2">
              Findings
            </h2>
          </v-card-title>
          <v-card-text class="pt-0">
            <v-layout row wrap>
              <v-flex xs12 sm10 md8 lg6>
                <v-expansion-panel popout>
                <v-expansion-panel-content :class="report.findings.ports.level.toLowerCase()">
                  <div slot="header" class="subheading">
                    <v-layout justify-space-between align-center row>
                      Ports
                      <v-btn outline color="blue" small :ripple="false">
                        {{ report.findings.ports.level.toLowerCase() }}
                      </v-btn>
                    </v-layout>
                  </div>
                  <v-card>
                    <v-divider />
                    <v-card-text>
                      <v-data-table
                      :headers="ports_headers"
                      :items="report.findings.ports.ports"
                      hide-actions
                      >
                        <template slot="items" slot-scope="props">
                          <td>{{ props.item.id }}</td>
                          <td>{{ props.item.protocol }}</td>
                          <td>{{ props.item.state }}</td>
                        </template>
                      </v-data-table>
                    </v-card-text>
                  </v-card>
                </v-expansion-panel-content>
               </v-expansion-panel>
              </v-flex>
            </v-layout>
          </v-card-text>
        </v-card>
      </v-flex>


    </v-layout>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import ReportsChart from './ReportsChart.vue';


@Component({
  components: {
    'blm-phaser-chart-reports': ReportsChart,
  },
})
export default class Report extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  report: any = null;
  reports: any[] = [];
  ports_headers = [
    {
      align: 'left',
      text: 'ID',
      value: 'id',
    },
    { text: 'Protocol', value: 'protocol' },
    { text: 'State', value: 'state' },
  ];


  // lifecycle
  created() {
    this.fetch_data(this.$route);
  }


  // watch
  @Watch('$route')
  on_route_change(to: any) {
    this.fetch_data(to);
  }


  // methods
  async fetch_data(route: any) {
    try {
      this.error = '';
      this.is_loading = true;
      const data = await api.query(
        api.PHASER_GRAPHQL,
        `query ($report_id: String!, $scan_id: String!) {
          scan(id: $scan_id) {
            reports {
              id
              created_at
              high_level_findings
              medium_level_findings
              low_level_findings
            }
          }
          report(id: $report_id) {
            id
            created_at
            started_at
            completed_at
            status
            duration
            high_level_findings
            medium_level_findings
            low_level_findings
            information_findings
            target
            profile
            trigger
            findings {
              ports {
                level
                ports {
                  id
                  state
                  protocol
                }
              }
            }
          }
        }
        `,
        {
          report_id: route.params.report_id,
          scan_id: route.params.scan_id,
        },
      );
      this.report = data.report;
      this.reports = data.scan.reports.reverse();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
.v-expansion-panel__container:first-child {
  border-top:1px solid #CFD8DC !important;
}

.v-expansion-panel__container.information {
  border-color: #CFD8DC #CFD8DC #CFD8DC #2196F3;
}
.v-expansion-panel__container {
  border-style: solid solid solid solid;
  border-width: 1px 1px 1px 4px;
  border-radius: 5px;
  border-color: #CFD8DC;
}
</style>
