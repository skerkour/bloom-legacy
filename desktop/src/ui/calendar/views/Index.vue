<template>
  <div class="fill-height">
    <v-row class="fill-height">

      <v-col sm="3" class="mb-4 pl-5 controls">

        <p class="mb-4 blm-pointer" @click="centerToday">
          {{ today }}
        </p>

        <!-- <v-row justify="center"> -->
        <v-btn
          color="primary"
          class="mb-4"
          @click="createEvent"
        >
          <v-icon left>mdi-plus</v-icon> New Event
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

        <!-- <br /><br />

        <v-select
          outlined
          v-model="type"
          :items="typeOptions"
        /> -->

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
          @change="calendarChanged"
        >

        <template v-slot:day="{ date }">
        <template v-for="event in eventsMap[date]">
          <div
            v-ripple
            class="blm-event"
            :key="`${event.id}${event.date}`"
            @click="editEvent(event)"
            >{{ event.title || '(No title)' }}</div>

        </template>
      </template>


        </v-calendar>
      </v-col>

    </v-row>

    <blm-calendar-dialog-event
      :visible="showEventDialog"
      :event="currentEvent"
      @closed="closeEventDialog"
      @created="eventCreated"
      @updated="eventUpdated"
      @deleted="eventDeleted"
    />
  </div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import moment from 'moment';
import EventDialog from '../components/EventDialog.vue';
import core from '@/core';
import {
  ListEvents, Event as EventModel, Events, Method,
} from '@/core/calendar';

@Component({
  components: {
    'blm-calendar-dialog-event': EventDialog,
  },
})
export default class Index extends Vue {
  // props
  // data
  type = 'month';
  now = moment().format('YYYY-MM-DD');
  focus = moment().format('YYYY-MM-DD');
  today = moment().format('dddd ll');
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
  showEventDialog = false;
  isLoading = false;
  error = '';
  events: EventModel[] = [];
  currentEvent: EventModel | null = null;

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
      for (let i = 1; i <= diff; i += 1) {
        const e2 = { ...e };
        e2.date = new Date(new Date(e2.start_at).setDate(new Date(e2.start_at).getDate() + i))
          .toISOString().substr(0, 10);
        (map[e2.date] = map[e2.date] || []).push(e);
      }
    });
    return map;
  }

  // lifecycle
  async created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData(startAt?: Date, endAt?: Date) {
    this.error = '';
    this.isLoading = true;
    const params: ListEvents = {
      start_at: startAt,
      end_at: endAt,
    };
    try {
      const res = await core.call(Method.ListEvents, params);
      this.events = (res as Events).events;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  createEvent() {
    this.currentEvent = null;
    this.openEventDialog();
  }

  centerToday() {
    this.focus = this.now;
  }

  editEvent(event: EventModel) {
    this.currentEvent = event;
    this.openEventDialog();
  }

  openEventDialog() {
    this.showEventDialog = true;
  }

  closeEventDialog() {
    this.showEventDialog = false;
  }

  eventCreated(event: EventModel) {
    this.events.push(event);
  }

  eventDeleted(event: EventModel) {
    this.events = this.events.filter((c: EventModel) => c.id !== event.id);
  }

  eventUpdated(updatedEvent: EventModel) {
    this.events = this.events.map((event: any) => {
      if (event.id === updatedEvent.id) {
        return updatedEvent;
      }
      return event;
    });
  }

  calendarChanged(to: any) {
    this.fetchData(
      new Date(to.start.date).toISOString() as unknown as Date,
      new Date(to.end.date).toISOString() as unknown as Date,
    );
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
