<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    @click:outside="close()"
    persistent
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>

      <v-card-title class="blue darken-1 white--text headline" v-if="event === null">
         <h3 class="headline mb-0">Create new event</h3>
      </v-card-title>
      <v-card-title dark class="blue darken-1 white--text headline" v-else>
        <h3 class="headline mb-0">
          <h3 class="headline mb-0">{{ event.title }}</h3>
        </h3>
        <v-spacer />
        <v-tooltip bottom>
          <v-menu slot="activator">
            <v-btn slot="activator" flat color="white" icon>
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>

            <v-list>
              <v-list-tile @click="deleteEvent">Delete</v-list-tile>
            </v-list>
          </v-menu>
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
              ref="start_at_menu"
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
              ref="end_at_menu"
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
import { Event as EventModel, GuiEvent } from '@/native/messages/calendar';
import { Native, Message } from '@/native';

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
  @Watch('startAt')
  onStartAtChanged(newStartAt: string) {
    const startAtTime = new Date(newStartAt).getTime();
    const endAtTime = new Date(this.endAt).getTime();
    if (endAtTime < startAtTime) {
      this.endAt = newStartAt;
    }
  }

  // methods
  async close() {
    await this.save();
    if (this.error !== '') {
      console.log('error not empty');
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
    } else if (this.title.trim().length !== 0 || this.description.trim().length !== 0
      || this.startAt !== this.now || this.endAt !== this.now) {
      await this.createEvent();
    }
  }

  clearFields() {
    this.title = '';
    this.description = '';
    this.startAt = this.now;
    this.endAt = this.now;
  }

  async deleteEvent() {
    this.error = '';
    console.log('delete Event');
    // this.is_loading = true;
    // try {
    //   await api.delete(`${api.CALENDAR}/v1/events/${event.id}`);
    //   this.$emit('delete', event);
    //   this.close();
    // } catch (err) {
    //   this.error = err.message;
    // } finally {
    //   this.is_loading = false;
    // }
  }

  async createEvent() {
    this.error = '';
    this.isLoading = true;
    const message: Message = {
      type: 'calendar.gui.create_event',
      data: {
        title: this.title,
        description: this.description,
        start_at: new Date(this.startAt).toISOString() as unknown as Date,
        end_at: new Date(this.endAt).toISOString() as unknown as Date,
      },
    };
    try {
      const res = await Native.call(message);
      this.$emit('created', (res.data as GuiEvent).event);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }

  async updateEvent() {
    console.log('update Event');
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
}
</script>


<style lang="scss" scoped>
</style>
