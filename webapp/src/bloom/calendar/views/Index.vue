<template>
<div class="fill-height">
  <v-layout row wrap class="blm-calendar-controls mt-2" align-center>

    <v-flex xs12 d-inline-block class="pr-4">
      <v-btn outline fab small color="primary" @click="$refs.calendar.prev()" class="mr-2">
        <v-icon dark>mdi-chevron-left</v-icon>
      </v-btn>
      <v-select
        v-model="type"
        :items="type_options"
        label="Type"
        dense
        :full-width="false"
      ></v-select>
      <v-btn outline fab small right color="primary" @click="$refs.calendar.next()" class="ml-4">
        <v-icon dark>mdi-chevron-right</v-icon>
      </v-btn>
      <v-btn outline right absolute color="primary" class="hidden-sm-and-down" @click="open_dialog">
        <v-icon left>mdi-plus</v-icon>Add
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


    <blm-calendar-dialog-event
      :event="current_event"
      :visible="event_dialog"
      @close="close_event_dialog"
      @create="event_created"
      @update="event_updated"
      @delete="event_deleted"
    />

  <v-btn @click="open_dialog" color="red" dark fab fixed bottom right class="hidden-md-and-up">
    <v-icon>mdi-plus</v-icon>
  </v-btn>
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import moment from 'moment';
import EventDialog from '../components/EventDialog.vue';


@Component({
  components: {
    'blm-calendar-dialog-event': EventDialog,
  },
})
export default class Calendar extends Vue {
  // props
  // data
  type = 'month';
  event_dialog = false;
  current_event: any | null = null;
  events: any[] = [];
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
    open_dialog() {
    this.event_dialog = true;
  }

  close_event_dialog() {
    this.event_dialog = false;
    this.current_event = null;
  }

  event_created(event: any) {
    this.events.push(event);
  }

  event_updated(updated_event: any) {
    this.events = this.events.map((event: any) => {
      return event.id === updated_event.id ? updated_event : event;
    });
  }

  event_deleted(deleted_event: any) {
    this.events = this.events.filter((c) => c.id !== deleted_event.id);
  }
}
</script>


<style scoped lang="scss">
.blm-calendar {
  height: calc(100% - 76px);
}
.blm-calendar-controls {
  height: 68px;
}

.v-select {
  display: inline-block !important;
}
</style>
