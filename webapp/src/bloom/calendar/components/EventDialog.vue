<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title class="blue darken-1 white--text headline" v-if="event === null">
         <h3 class="headline mb-0">Create new event</h3>
      </v-card-title>
      <v-card-title primary-title v-else>
        <h3 class="headline mb-0">
          <h3 class="headline mb-0">{{ event.title }}</h3>
        </h3>
        <v-spacer />
        <v-tooltip bottom>
          <v-menu slot="activator">
            <v-btn slot="activator" flat color="blue-grey" icon>
              <v-icon>mdi-dots-vertical</v-icon>
            </v-btn>

            <v-list>
              <v-list-tile @click="delete_event(event)">Delete</v-list-tile>
            </v-list>
          </v-menu>
        <span>More actions</span>
        </v-tooltip>
      </v-card-title>
      <v-divider />
      <v-card-text>
        <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
          {{ error }}
        </v-alert>
        <v-container fluid grid-list-lg>
          <v-layout row wrap>

            <v-flex xs12>
              <v-text-field
                label="Title"
                v-model="title"
                :readonly="viewing"
                outline
              ></v-text-field>
            </v-flex>


            <!-- start_at -->
            <v-flex xs12>
              <v-menu
                ref="start_at_menu"
                v-model="start_at_menu"
                :close-on-content-click="false"
                :nudge-right="40"
                :return-value.sync="start_at"
                lazy
                transition="scale-transition"
                offset-y
                full-width
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    v-model="start_at"
                    label="Start at"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-on="on"
                  ></v-text-field>
                </template>
                <v-date-picker v-model="start_at" no-title scrollable>
                  <v-spacer></v-spacer>
                  <v-btn flat color="primary" @click="start_at_menu = false">Cancel</v-btn>
                  <v-btn flat color="primary" @click="$refs.start_at_menu.save(start_at)">OK</v-btn>
                </v-date-picker>
              </v-menu>
            </v-flex>


            <!-- end_at -->
            <v-flex xs12>
              <v-menu
                ref="end_at_menu"
                v-model="end_at_menu"
                :close-on-content-click="false"
                :nudge-right="40"
                :return-value.sync="end_at"
                lazy
                transition="scale-transition"
                offset-y
                full-width
                min-width="290px"
              >
                <template v-slot:activator="{ on }">
                  <v-text-field
                    v-model="end_at"
                    label="End at"
                    prepend-icon="mdi-calendar"
                    readonly
                    v-on="on"
                  ></v-text-field>
                </template>
                <v-date-picker v-model="end_at" no-title scrollable>
                  <v-spacer></v-spacer>
                  <v-btn flat color="primary" @click="end_at_menu = false">Cancel</v-btn>
                  <v-btn flat color="primary" @click="$refs.end_at_menu.save(end_at)">OK</v-btn>
                </v-date-picker>
              </v-menu>
            </v-flex>

            <!-- description -->
            <v-flex xs12>
              <v-textarea
                label="Description"
                v-model="description"
                :readonly="viewing"
                outline
              ></v-textarea>
            </v-flex>

          </v-layout>
        </v-container>
      </v-card-text>
      <v-divider />
      <v-card-actions>
        <v-spacer></v-spacer>
        <div v-if="creating || updating">
          <v-btn flat @click="cancel" :disabled="is_loading">Cancel</v-btn>
          <v-btn color="primary" :loading="is_loading" @click="save_event">
            Create
          </v-btn>
        </div>
        <div v-else-if="viewing">
          <v-btn flat @click="close" :disabled="is_loading">Close</v-btn>
          <v-btn color="primary" :loading="is_loading" @click="editing = true">
            Edit
          </v-btn>
        </div>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue, Watch } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';

const DEFAULT_EMAIL = { email: null, label: 'OTHER' };
const DEFAULT_WEBSITE = { website: null, label: 'OTHER' };
const DEFAULT_PHONE = { phone: null, label: 'OTHER' };
const DEFAULT_ORGANIZATION = { name: null, title: null };

@Component
export default class EventDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;
  @Prop({ type: Object, default: null }) event!: any | null;


  // data
  error = '';
  is_loading = false;
  editing = false;
  // event data
  title = '';
  description = '';

  start_at = new Date().toISOString().substr(0, 10);
  start_at_menu = false;

  end_at = new Date().toISOString().substr(0, 10);
  end_at_menu = false;


  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
    }
  }

  get creating() {
    return this.visible && this.event === null;
  }

  get updating() {
    return this.visible && this.event !== null && this.editing;
  }

  get viewing() {
     return this.visible && this.event && this.editing === false;
  }


  // lifecycle
  // watch

  @Watch('event')
  on_event_changed(event: any) {
    if (event !== null) {
      this.title = event.title;
      this.description = event.description;
      this.start_at = event.start_at;
      this.end_at = event.end_at;
    } else {
      this.clear_fields();
    }
  }

  @Watch('birthday_menu')
  on_birthday_menu_changed(birthday_menu: boolean) {
    if (birthday_menu) {
      this.$nextTick(() => ((this.$refs.birthday_picker) as any).activePicker = 'YEAR');
    }
  }


  // methods
  close() {
    this.show = false;
    this.start_at_menu = false;
    this.editing = false;
    this.error = '';
    this.is_loading = false;
    this.clear_fields();
  }

  cancel() {
    if (this.updating) {
      this.editing = false;
    } else {
      this.close();
    }
  }

  clear_fields() {
    this.title = '';
    this.description = '';
    this.start_at = new Date().toISOString().substr(0, 10);
    this.end_at = new Date().toISOString().substr(0, 10);
  }

  save_event() {
    this.create_event();
  }

  save_birthday(date: any) {
    (this.$refs.birthday_menu as any).save(date);
  }

  async delete_event(event: any) {
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.CALENDAR}/v1/events/${event.id}`);
      this.$emit('delete', event);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async create_event() {
    this.error = '';
    this.is_loading = true;

    const start_at  = new Date(this.start_at);


    const payload = {
      description: this.description,
      end_at: new Date(this.end_at),
      start_at: new Date(this.start_at),
      title: this.title,
    };

    try {
      const res = await api.post(`${api.CALENDAR}/v1/events`, payload);
      res.birthday = res.birthday
        ? new Date(res.birthday).toISOString().slice(0, -14) : res.birthday;
      this.$emit('create', res);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  format_date(date: any) {
    if (!date) {
      return null;
    }

    const [year, month, day] = date.split('-');
    return `${year}/${month}/${day}`;
  }

  parse_date(date: any) {
    if (!date) {
      return null;
    }

    const [year, month, day] = date.split('/');
    return `${year}-${month.padStart(2, '0')}-${day.padStart(2, '0')}`;
  }
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}

.events-add-row {
  .v-list__tile {
    padding-left: 8px;
  }
}
</style>
