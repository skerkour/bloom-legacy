<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close(false)"
    @click:outside="close(false)"
    persistent
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>

      <v-card-title class="headline" v-if="event === null">
         <h3 class="headline mb-0">Create new event</h3>
      </v-card-title>
      <v-card-title dark class="headline" v-else>
        <h3 class="headline mb-0">
          <h3 class="headline mb-0">{{ event.title }}</h3>
        </h3>
        <v-spacer />
        <v-tooltip bottom>
          <template v-slot:activator="{ on: tooltip }">
            <v-menu slot="activator">
              <template v-slot:activator="{ on: menu }">
              <v-btn
                slot="activator"
                text
                icon
                v-on="{ ...tooltip, ...menu }"
              >
                <v-icon>mdi-dots-vertical</v-icon>
              </v-btn>
              </template>

              <v-list>
                <v-list-item @click="deleteEvent">
                  <v-list-item-title>Delete Event</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </template>
          <span>More actions</span>
        </v-tooltip>
      </v-card-title>

      <v-card-text>
        <v-container fluid grid-list-lg>
          <v-alert icon="mdi-alert-circle" :value="error !== ''" type="error" dismissible>
            {{ error }}
          </v-alert>

          <v-col>
            <v-text-field
              label="Title"
              v-model="title"
              outlined
            />
          </v-col>

          <v-col>
            <v-menu
              ref="startAt_menu"
              v-model="startAtMenu"
              :close-on-content-click="false"
              :nudge-right="40"
              transition="scale-transition"
              offset-y
              min-width="290px"
            >
              <template v-slot:activator="{ on }">
                <v-text-field
                  v-model="startAt"
                  label="Start at"
                  prepend-icon="mdi-calendar"
                  readonly
                  v-on="on"
                />
              </template>
              <v-date-picker v-model="startAt" @input="startAtMenu = false"></v-date-picker>
            </v-menu>
          </v-col>

          <v-col>
            <v-menu
              ref="endAt_menu"
              v-model="endAtMenu"
              :close-on-content-click="false"
              :nudge-right="40"
              transition="scale-transition"
              offset-y
              min-width="290px"
            >
              <template v-slot:activator="{ on }">
                <v-text-field
                  v-model="endAt"
                  label="Start at"
                  prepend-icon="mdi-calendar"
                  readonly
                  v-on="on"
                />
              </template>
              <v-date-picker v-model="endAt" @input="endAtMenu = false"></v-date-picker>
            </v-menu>
          </v-col>

          <v-col>
            <v-textarea
              label="Description"
              v-model="description"
              outlined
            />
          </v-col>

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
import core from '@/core';
import {
  DeleteEvent, CreateEvent, Event as EventModel, Method,
} from '@/core/calendar';
import { log } from '@/libs/rz';


@Component
export default class EventDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) event!: EventModel | null;

  // data
  title = '';
  description = '';
  now = new Date().toISOString().substr(0, 10);
  startAt = new Date().toISOString().substr(0, 10);
  startAtMenu = false;
  endAt = new Date().toISOString().substr(0, 10);
  endAtMenu = false;
  error = '';
  isLoading = false;

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('closed');
    }
  }

  // lifecycle
  // watch
  @Watch('event')
  onEventChanged(event: any) {
    if (event !== null) {
      this.title = event.title;
      this.description = event.description;
      this.startAt = event.startAt;
      this.endAt = event.endAt;
    } else {
      this.clearFields();
    }
  }

  @Watch('startAt')
  onStartAtChanged(newStartAt: string) {
    const startAtTime = new Date(newStartAt).getTime();
    const endAtTime = new Date(this.endAt).getTime();
    if (endAtTime < startAtTime) {
      this.endAt = newStartAt;
    }
  }

  // methods
  async close(deleted: boolean) {
    if (!deleted) {
      await this.save();
    }
    if (this.error !== '') {
      log.debug('error not empty');
      return;
    }
    this.show = false;
    this.startAtMenu = false;
    this.endAtMenu = false;
    this.error = '';
    this.isLoading = false;
    this.clearFields();
  }

  async save() {
    if (this.event) {
      await this.updateEvent();
    } else {
      await this.createEvent();
    }
  }

  clearFields() {
    this.title = '';
    this.description = '';
    this.startAt = this.now;
    this.endAt = this.now;
  }

  async createEvent() {
    if (this.isEmpty()) {
      return;
    }
    this.error = '';
    this.isLoading = true;
    const params: CreateEvent = {
      title: this.title,
      description: this.description,
      startAt: core.toIsoDate(this.startAt)!,
      endAt: core.toIsoDate(this.endAt)!,
    };
    try {
      const res = await core.call(Method.CreateEvent, params);
      this.$emit('created', (res as Event));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async updateEvent() {
    this.error = '';
    this.isLoading = true;
    const event = { ...this.event } as EventModel;
    event.title = this.title;
    event.description = this.description;
    event.startAt = core.toIsoDate(this.startAt)!;
    event.endAt = core.toIsoDate(this.endAt)!;
    try {
      const res = await core.call(Method.UpdateEvent, event);
      this.$emit('updated', (res as Event));
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async deleteEvent() {
    this.error = '';
    this.isLoading = true;
    const params: DeleteEvent = {
      id: this.event!.id,
    };
    try {
      await core.call(Method.DeleteEvent, params);
      this.$emit('deleted', this.event);
      this.close(true);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
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

  isEmpty(): boolean {
    if (this.title.trim().length !== 0 || this.description.trim().length !== 0
      || this.startAt !== this.now || this.endAt !== this.now) {
      return false;
    }
    return true;
  }
}
</script>


<style lang="scss" scoped>
</style>
