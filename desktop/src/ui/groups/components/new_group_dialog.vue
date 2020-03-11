<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    @click:outside="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >
    <v-card>
      <v-card-title>
        <h2 class="headline">New Group</h2>
      </v-card-title>

      <v-card-text>
        <v-col cols="12" v-if="error">
          <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
            {{ error }}
          </v-alert>
        </v-col>
        <v-col cols="12" v-if="loading">
          <v-progress-circular
            indeterminate
            color="primary"
            size="50"
          />
        </v-col>

        <v-col cols="12">
          <v-text-field label="Name" v-model="name" :disabled="loading" />
        </v-col>

        <v-col cols="12">
          <v-textarea
            label="Description"
            v-model="description"
            :disabled="loading" />
        </v-col>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="cancel()" :loading="loading">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="createGroup()" :loading="loading">
          Add
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import { CreateGroupInput, Maybe, Group } from '@/api/models';
import core from '@/core';
import { Method } from '@/core/groups';

@Component
export default class Groups extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;

  // data
  error = '';
  loading = false;
  name = '';
  description = '';

  // computed
  get show() {
    return this.visible;
  }

  set show(value) {
    if (!value) {
      this.$emit('closed');
    }
  }

  // computed
  // lifecycle
  // watch
  // methods
  async createGroup() {
    this.loading = true;
    this.error = '';
    const params: CreateGroupInput = {
      name: this.name,
      description: this.description,
      usersToInvite: [],
    };

    try {
      const res: Maybe<Group> = await core.call(Method.CreateGroup, params);
      this.$emit('created', res);
      this.close();
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }

  cancel() {
    this.name = '';
    this.description = '';
    this.close();
  }

  async close() {
    this.show = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
