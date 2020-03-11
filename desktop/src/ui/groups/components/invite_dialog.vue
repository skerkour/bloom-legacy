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
        <h2 class="headline">Invite people</h2>
      </v-card-title>

      <v-card-text>
        <v-col cols="12" v-if="error">
          <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
            {{ error }}
          </v-alert>
          <v-progress-circular
            indeterminate
            color="primary"
            size="50"
          />
        </v-col>

      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="cancel()" :loading="loading">
          Cancel
        </v-btn>
        <v-btn color="primary" @click="invite()" :loading="loading">
          Invite
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';

@Component
export default class Groups extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;

  // data
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

  // computed
  // lifecycle
  // watch
  // methods
  async invite() {
    console.log('invited');
    this.close();
  }

  cancel() {
    this.close();
  }

  async close() {
    this.show = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
