<template>
<div class="fill-height">
  <v-layout row wrap class="blm-calendar-controls mt-2" align-center>

    <v-flex xs12 class="pr-4 text-xs-center text-sm-left">
      <v-btn  outline fab small color="primary" @click="$refs.calendar.prev()" class="mr-2">
        <v-icon dark>mdi-chevron-left</v-icon>
      </v-btn>
      <v-select
        v-model="type"
        :items="type_options"
        label="Type"
        dense
        d-inline-block
        :full-width="false"
      ></v-select>
      <v-btn outline fab small right color="primary" @click="$refs.calendar.next()" class="ml-4">
        <v-icon dark>mdi-chevron-right</v-icon>
      </v-btn>
      <v-btn outline right absolute color="primary" class="hidden-xs-only" @click="open_dialog">
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
      >
      <template v-slot:day="{ date }">
        <template v-for="event in eventsMap[date]">
          <div
            v-ripple
            class="blm-event"
            :key="`${event.id}${event.date}`"
            @click="edit_event(event)"
            >{{ event.title || '(No title)' }}</div>

        </template>
      </template>

      </v-calendar>
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

  <v-btn @click="open_dialog" color="red" dark fab fixed bottom right class="hidden-sm-and-up">
    <v-icon>mdi-plus</v-icon>
  </v-btn>
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import moment from 'moment';
import api from '@/bloom/kernel/api';
import EventDialog from '../components/EventDialog.vue';


@Component({
  components: {
    'blm-calendar-dialog-event': EventDialog,
  },
})
export default class Calendar extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
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
  get eventsMap() {
    const map: any = {};
    this.events.forEach((e: any) => {
      e.start_at = new Date(e.start_at).toISOString().substr(0, 10);
      e.end_at = new Date(e.end_at).toISOString().substr(0, 10);
      e.date = new Date(e.start_at).toISOString().substr(0, 10);
      (map[e.date] = map[e.date] || []).push(e);

      // because vuetify does not support multi day events
      const diff = moment(e.end_at).diff(e.start_at, 'days');
      for (let i = 1; i <= diff; i++) {
        const e2 = Object.assign({}, e);
        e2.date = new Date(new Date(e2.start_at).setDate(new Date(e2.start_at).getDate() + i))
          .toISOString().substr(0, 10);
        (map[e2.date] = map[e2.date] || []).push(e);
      }
    });
    return map;
  }


  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    this.is_loading = true;
    try {
      this.events = await api.get(`${api.CALENDAR}/v1/events`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }


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

  edit_event(event: any) {
    this.current_event = event;
    this.open_dialog();
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
  max-width: 250px;
}

.blm-event {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  border-radius: 2px;
  background-color: #1867c0;
  color: #ffffff;
  border: 1px solid #1867c0;
  width: 100%;
  font-size: 12px;
  padding: 3px;
  cursor: pointer;
  margin-bottom: 1px;
}
</style>
