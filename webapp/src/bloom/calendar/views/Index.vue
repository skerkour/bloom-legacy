<template>
<div class="fill-height">
  <v-layout row wrap class="blm-calendar-controls mb-2 mt-2">
    <v-flex xs4 class="text-xs-right pr-4">
      <v-btn outline color="primary" @click="$refs.calendar.prev()">
        <v-icon dark left>
          mdi-chevron-left
        </v-icon>
        Prev
      </v-btn>
    </v-flex>
    <v-flex xs4 class="text-xs-center">
      <v-select
        v-model="type"
        :items="type_options"
        label="Type"
      ></v-select>
    </v-flex>
    <v-flex xs4 class="text-xs-left pl-4">
      <v-btn outline color="primary" @click="$refs.calendar.next()">
        Next
        <v-icon right dark>
          mdi-chevron-right
        </v-icon>
      </v-btn>
    </v-flex>
  </v-layout>

  <v-layout wrap class="blm-calendar">
    <v-flex xs12>
      <v-calendar
        ref="calendar"
        :type="type"
        color="primary"
        :weekdays="weekdays"
        v-model="current_day"
        :end="end"
      ></v-calendar>
    </v-flex>
  </v-layout>
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import moment from 'moment';


@Component
export default class Calendar extends Vue {
  // props
  // data
  type = 'month';
  current_day = moment().format('YYYY-MM-DD');
  end = moment().endOf('month').format('YYYY-MM-DD');
  type_options = [
    { text: 'Day', value: 'day' },
    { text: 'Week', value: 'week' },
    { text: 'Month', value: 'month' },
  ];
  weekdays = [1, 2, 3, 4, 5, 6, 0];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
.blm-calendar {
  height: calc(100% - 60px);
}
.blm-calendar-controls {
  height: 60px;
}
</style>
