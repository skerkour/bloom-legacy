<template>
  <v-dialog
    v-model="show"
    @keydown.esc="close()"
    @click:outside="close()"
    scrollable
    width="50%"
    :fullscreen="$vuetify.breakpoint.smAndDown"
  >

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
  isLoading = false;
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
  async close() {
    this.show = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
