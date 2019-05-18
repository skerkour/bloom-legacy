<template>
  <v-dialog
  v-model="show"
  max-width="400px"
  @keydown.esc="show = false"
  >
    <v-card>
      <v-card-title class="blue darken-1 white--text headline">
        New scan
      </v-card-title>

      <v-card-text>
        <v-container>
          <v-layout row wrap>

            <v-flex xs12>
              <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
                {{ error }}
              </v-alert>
            </v-flex>

            <v-flex xs12>
              <v-text-field
                label="Name"
                v-model="name"
              />
            </v-flex>

            <v-flex xs12>
              <v-textarea
                label="Description"
                v-model="description"
                hint="Optional"
              />
            </v-flex>

            <v-flex xs12 class="pt-3">
              <v-select
                :items="profiles"
                box
                label="Profile"
                v-model="profile"
              />
            </v-flex>

            <v-flex xs12 class="pt-3">
              <v-select
                :items="schedule_enum"
                box
                label="Schedule"
                v-model="schedule"
              />
            </v-flex>

            <v-flex xs12>
              <v-text-field
                label="Target"
                placeholder="Ip or domain: 192.168.1.1, example.com..."
                v-model="target"
              />
            </v-flex>

          </v-layout>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn flat @click="close" :disabled="is_loading">Cancel</v-btn>
        <v-btn
          @click="create"
          color="primary"
          :loading="is_loading"
          :disabled="!can_create">
          Create
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
import { Profile, Schedule } from '../models/Scan';


const DEFAULT_PROFILE = Profile.Network;
const DEFAULT_SCHEDULE = Schedule.Daily;

@Component
export default class NewScanDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;


  // data
  error = '';
  is_loading = false;
  profiles = [Profile.Network, Profile.Application];
  schedule_enum = [Schedule.Daily, Schedule.Weekly, Schedule.Monthly, Schedule.Never];
  name = '';
  description = '';
  profile: Profile = DEFAULT_PROFILE;
  schedule: Schedule = DEFAULT_SCHEDULE;
  target = '';

  // computed
  get can_create() {
    return this.name !== '' && this.target !== '';
  }

  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('close');
    }
  }


  // lifecycle
  // watch
  // methods
  close() {
    this.reset();
    this.show = false;
  }

  async create() {
    this.error = '';
    this.is_loading = true;
    const payload = {
      description: this.description,
      name: this.name,
      profile: this.profile,
      schedule: this.schedule,
      target: this.target,
    };
    try {
      const res = await api.post(`${api.PHASER}/v1/scans`, payload);
      this.$emit('create', res);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  reset() {
    this.name = '';
    this.error = '';
    this.profile = DEFAULT_PROFILE;
    this.schedule = DEFAULT_SCHEDULE;
    this.target = '';
    this.description = '';
  }
}
</script>
