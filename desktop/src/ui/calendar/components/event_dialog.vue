<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close"
    @click:outside="close"
    persistent
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.xsOnly"
  >
    <v-card>

      <v-card-title class="headline" v-if="event === null">
         <h3 class="headline mb-0">Create new event</h3>
         <v-spacer />
        <v-btn text @click="cancel">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="createEvent">
          Create
        </v-btn>
      </v-card-title>
      <v-card-title dark class="headline" v-else>
        <h3 class="headline mb-0">
          <h3 class="headline mb-0">{{ event.data.title }}</h3>
        </h3>
        <v-spacer />
        <v-btn text @click="cancel">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="updateEvent">
          Save
        </v-btn>
        <v-menu bottom left>
          <template v-slot:activator="{ on }">
            <v-btn icon v-on="on">
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>
          </template>

          <v-list>
            <v-list-item @click="deleteEvent">
              <v-list-item-icon>
                <v-icon>mdi-delete</v-icon>
              </v-list-item-icon>
              <v-list-item-title>Delete event</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu>
      </v-card-title>

      <v-card-text>
        <v-container fluid grid-list-lg>
          <v-row>
            <v-col cols="12">
              <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
                {{ error }}
              </v-alert>
            </v-col>

            <v-col cols="12">
              <v-text-field label="Title" v-model="title" outlined/>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="startAtDateMenu"
                v-model="startAtDateMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    :value="formattedStartAt"
                    label="Start at"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-date-picker v-model="vuetifyStartAt" @input="startAtDateMenu = false" />
              </v-menu>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="startAtTimeMenu"
                v-model="startAtTimeMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    :value="startAtTime"
                    label="Start at"
                    prepend-icon="mdi-clock-outline"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-time-picker
                  v-model="startAtTime"
                  format="24hr"
                  @click:minute="startAtTimeMenu = false"
                ></v-time-picker>
              </v-menu>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="endAtDateMenu"
                v-model="endAtDateMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    :value="formattedEndAt"
                    label="End at"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-date-picker v-model="vuetifyEndAt" @input="endAtDateMenu = false" />
              </v-menu>
            </v-col>

            <v-col cols="6">
              <v-menu
                ref="endAtTimeMenu"
                v-model="endAtTimeMenu"
                :close-on-content-click="false"
                :nudge-right="40"
                transition="scale-transition"
                offset-y
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    v-model="endAtTime"
                    label="End at"
                    prepend-icon="mdi-clock-outline"
                    readonly
                    v-on="on"
                  />
                </template>
                <v-time-picker
                  v-model="endAtTime"
                  format="24hr"
                  @click:minute="endAtTimeMenu = false"
                ></v-time-picker>
              </v-menu>
            </v-col>

            <v-col cols="12">
              <v-textarea
                label="Description"
                v-model="description"
                outlined
              />
            </v-col>

          </v-row>
        </v-container>
      </v-card-text>
    </v-card>
 </v-dialog>

</template>


<script lang="ts">
import {
  Component,
  Prop,
  Vue,
  Watch,
} from 'vue-property-decorator';
import moment from 'moment';
import core, { BlmObject } from '@/core';
import { Event as EventModel, Method } from '@/core/calendar';
import { CalendarCreateEventParams, CalendarDeleteEventParams } from '@/core/messages';

@Component
export default class EventDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) event!: BlmObject<EventModel> | null;

  // data
  title = '';
  description = '';
  now = new Date();
  startAt = new Date();
  startAtTime = '08:00';
  startAtDateMenu = false;
  startAtTimeMenu = false;
  endAt = new Date();
  endAtTime = '09:00';
  endAtDateMenu = false;
  endAtTimeMenu = false;
  error = '';
  loading = false;

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('closed');
    }
  }

  get formattedStartAt(): string {
    return this.startAt ? moment(this.startAt).format('dddd, MMMM Do YYYY') : '';
  }

  get formattedEndAt(): string {
    return this.endAt ? moment(this.endAt).format('dddd, MMMM Do YYYY') : '';
  }

  get vuetifyStartAt(): string {
    return this.startAt.toISOString().substr(0, 10);
  }

  set vuetifyStartAt(value: string) {
    this.startAt = new Date(value);
  }

  get vuetifyEndAt(): string {
    return this.endAt.toISOString().substr(0, 10);
  }

  set vuetifyEndAt(value: string) {
    this.endAt = new Date(value);
  }

  // lifecycle
  // watch
  @Watch('event')
  onEventChanged(event: BlmObject<EventModel>) {
    if (event !== null) {
      this.title = event.data.title;
      this.description = event.data.description;
      this.startAt = event.data.startAt;
      this.startAtTime = this.dateToTimeSring(event.data.startAt);
      this.endAt = event.data.endAt;
      this.endAtTime = this.dateToTimeSring(event.data.endAt);
    } else {
      this.emptyFields();
    }
  }

  @Watch('startAt')
  onStartAtChanged(newStartAt: Date) {
    if (!newStartAt) {
      return;
    }
    const startAtTime = newStartAt.getTime();
    const endAtTime = this.endAt.getTime();
    if (endAtTime < startAtTime) {
      this.endAt = newStartAt;
    }
  }

  @Watch('startAtTime')
  onStartAtTimeChanged(newStartTime: string) {
    if (!newStartTime) {
      return;
    }
    const startAtTime = this.timeToDate(this.startAt, newStartTime).getTime();
    const endAtTime = this.timeToDate(this.endAt, this.endAtTime).getTime();
    if (endAtTime < startAtTime) {
      this.endAtTime = newStartTime;
    }
  }

  // methods
  cancel() {
    this.close();
    this.emptyFields();
  }

  async close() {
    this.show = false;
    this.startAtDateMenu = false;
    this.startAtTimeMenu = false;
    this.endAtDateMenu = false;
    this.endAtTimeMenu = false;
    this.error = '';
    this.loading = false;
  }

  emptyFields() {
    this.title = '';
    this.description = '';
    this.startAt = this.now;
    this.endAt = this.now;
  }

  async createEvent() {
    this.error = '';
    this.loading = true;
    const startAt = this.timeToDate(this.startAt, this.startAtTime);
    const endAt = this.timeToDate(this.endAt, this.endAtTime);
    const params: CalendarCreateEventParams = {
      location: '',
      title: this.title,
      description: this.description,
      startAt,
      endAt,
    };

    try {
      const res = await core.call(Method.CreateEvent, params);
      this.$emit('created', (res as BlmObject<EventModel>));
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async updateEvent() {
    this.error = '';
    this.loading = true;
    const event = { ...this.event } as BlmObject<EventModel>;
    event.data.title = this.title;
    event.data.description = this.description;
    event.data.startAt = this.timeToDate(this.startAt, this.startAtTime);
    event.data.endAt = this.timeToDate(this.endAt, this.endAtTime);
    try {
      const res = await core.call(Method.UpdateEvent, event);
      this.$emit('updated', (res as BlmObject<EventModel>));
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  async deleteEvent() {
    this.error = '';
    this.loading = true;
    const params: CalendarDeleteEventParams = {
      id: this.event!.id,
    };
    try {
      await core.call(Method.DeleteEvent, params);
      this.$emit('deleted', this.event);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }


  formatDate(date: string) {
    if (!date) {
      return null;
    }

    const [year, month, day] = date.split('-');
    return `${year}/${month}/${day}`;
  }

  parseDate(date: string) {
    if (!date) {
      return null;
    }

    const [year, month, day] = date.split('/');
    return `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
  }

  timeToDate(date: Date, time: string): Date {
    const ret = date;
    ret.setHours(parseInt(time[0] + time[1], 10));
    ret.setMinutes(parseInt(time[3] + time[4], 10));
    return ret;
  }

  dateToTimeSring(date: Date): string {
    const hours = date.getHours().toString();
    const minutes = date.getMinutes().toString();
    return `${hours.padStart(2, '0')}:${minutes.padStart(2, '0')}`;
  }
}
</script>


<style lang="scss" scoped>
</style>
