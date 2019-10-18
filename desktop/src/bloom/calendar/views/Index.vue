<template>
  <div class="fill-height">
    <v-row class="fill-height">

      <v-col
        sm="3"
        class="mb-4 pl-5 controls"
      >

        <p
          class="mb-4 blm-pointer"
          @click="centerToday"
        >
          {{ today }}
        </p>

        <!-- <v-row justify="center"> -->
        <v-btn
          outlined
          large
          color="primary"
          class="mb-4"
        >
          New Event
        </v-btn>
        <!-- </v-row> -->

        <v-btn
          fab
          outlined
          small
          absolute
          left
          color="primary"
          @click="$refs.calendar.prev()"
        >
          <v-icon dark>mdi-chevron-left</v-icon>
        </v-btn>

        <v-btn
          fab
          outlined
          small
          absolute
          right
          color="primary"
          @click="$refs.calendar.next()"
        >
          <v-icon dark>mdi-chevron-right</v-icon>
        </v-btn>

        <br /><br />

        <v-select
          outlined
          v-model="type"
          :items="typeOptions"
        ></v-select>

      </v-col>

      <v-col
        sm="9"
        class="col-no-padding"
      >

        <v-calendar
          ref="calendar"
          v-model="focus"
          :type="type"
          :start="start"
          :end="end"
          :now="now"
        />
      </v-col>

    </v-row>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import moment from 'moment';

@Component
export default class Index extends Vue {
  // props
  // data
  type = 'month';
  now = moment().format('YYYY-MM-DD');
  focus = moment().format('YYYY-MM-DD');
  today = moment().format('LLLL');
  typeOptions = [
    { text: 'Day', value: 'day' },
    { text: '4 Day', value: '4day' },
    { text: 'Week', value: 'week' },
    { text: 'Month', value: 'month' },
  ];
  start = moment()
    .startOf('month')
    .format('YYYY-MM-DD');
  end = moment()
    .endOf('month')
    .format('YYYY-MM-DD');

  // computed
  // lifecycle
  // watch
  // methods
  centerToday() {
    this.focus = this.now;
  }
}
</script>


<style lang="scss" scoped>
.controls {
  position: relative;
}

.col-no-padding {
  padding: 0;
}
</style>
