<template>
  <v-container fluid class="text-left">

    <v-row>
      <v-col cols="12">
        <div class="headline">
          Reports
        </div>
      </v-col>
    </v-row>


    <v-row><v-col cols="12"><v-divider /></v-col></v-row>


    <v-row>
      <v-col cols="12">
        <div class="headline">
          Overview
        </div>
      </v-col>
    </v-row>

    <v-row>
      <v-col>
        <v-container>
          <v-row>
            <v-col cols="12" class="text-center">
                <p class="title blue--text">{{ report.total_findings }}</p>
                <p class="title">Findings</p>
            </v-col>
          </v-row>
          <v-row>

            <v-col cols="3">
              <v-subheader>Started</v-subheader>
              <p>{{ report.started_at | date }}</p>
            </v-col>

            <v-col cols="3">
              <v-subheader>Completed</v-subheader>
              <p>{{ report.completed_at | date }}</p>
            </v-col>

            <v-col cols="3">
              <v-subheader>Duration</v-subheader>
              <p>{{ subDates(report.started_at, report.completed_at) | duration }}</p>
            </v-col>

            <v-col cols="3">
              <v-subheader>Target</v-subheader>
              <p>{{ report.target }}</p>
            </v-col>
          </v-row>

          <v-row>
            <v-col cols="3">
              <v-subheader>Profile</v-subheader>
              <p>{{ report.profile }}</p>
            </v-col>

            <v-col cols="3">
              <v-subheader>Trigger</v-subheader>
              <p>{{ report.trigger }}</p>
            </v-col>

            <v-col cols="3">
              <v-subheader>Status</v-subheader>
              <p>{{ report.status }}</p>
            </v-col>
          </v-row>
        </v-container>
      </v-col>
    </v-row>


    <v-row><v-col cols="12"><v-divider /></v-col></v-row>


    <v-row>
      <v-col cols="12">
        <div class="headline">
          Findings
        </div>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Report extends Vue {
  // props
  // data
  report = {
    id: '1',
    total_findings: 42,
    started_at: new Date(Date.now() - 10000 * 60),
    completed_at: new Date(),
    target: 'kerkour.fr',
    trigger: 'Manual',
    profile: 'Application',
    status: 'Success',
  };

  // computed
  // lifecycle
  // watch
  // methods
  subDates(a: Date, b: Date): number {
    return Math.abs(a.getTime() - b.getTime()) / 1000;
  }
}
</script>


<style lang="scss" scoped>
.v-subheader {
  padding: 0;
}
</style>
