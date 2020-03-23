<template>
  <div class="fill-height">
    <v-row class="fill-height">
      <v-col cols="3">
        <v-row>
          <v-col cols="3">
            <v-btn fab outlined small color="primary" @click="$refs.calendar.prev()">
              <v-icon dark>mdi-chevron-left</v-icon>
            </v-btn>
          </v-col>

          <v-col sm="6" class="controls">
            <p class="mb-4 blm-pointer" @click="centerToday">
              {{ today }}
            </p>
          </v-col>

          <v-col cols="3">
            <v-btn fab outlined small color="primary" @click="$refs.calendar.next()">
              <v-icon dark>mdi-chevron-right</v-icon>
            </v-btn>
          </v-col>

          <v-col cols="12">
            <v-btn color="primary" @click="createEvent">
              <v-icon left>mdi-plus</v-icon> New Event
            </v-btn>
          </v-col>

        <!-- <v-select
          outlined
          v-model="type"
          :items="typeOptions"
        /> -->
        </v-row>
      </v-col>

      <v-col sm="9" class="col-no-padding">

        <v-calendar
          ref="calendar"
          v-model="focus"
          :type="type"
          :start="start"
          :end="end"
          :now="now"
          color="error"
          @change="calendarChanged"
          @click:event="editEvent"
          :events="vuetifyEvents"
        >

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
  get vuetifyEvents(): any[] {
    return this.events.map((event: any) => { // eslint-disable-line
      event.startAt = new Date(event.startAt); // eslint-disable-line
      event.endAt = new Date(event.endAt); // eslint-disable-line
      event.start = this.formatDate(event.startAt); // eslint-disable-line
      event.end = this.formatDate(event.endAt); // eslint-disable-line
      event.name = event.title || '(No title)'; // eslint-disable-line
      return event;
    });
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
      startAt,
      endAt,
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

  editEvent(event: any) {
    this.currentEvent = event.event;
    this.openEventDialog();
  }

  openEventDialog() {
    this.showEventDialog = true;
  }

  closeEventDialog() {
    this.showEventDialog = false;
    this.currentEvent = null;
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

  formatDate(date: Date) {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()} ${date.getHours()}:${date.getMinutes()}`;
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
