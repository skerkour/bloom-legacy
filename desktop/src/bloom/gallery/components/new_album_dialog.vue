<template>
  <v-dialog v-model="show" max-width="400">
    <v-card>
      <v-card-title class="headline">New Album</v-card-title>

      <v-card-actions>
        <v-spacer />

        <v-btn text @click="show = false">
          Cancel
        </v-btn>

        <v-btn color="primary" @click="show = false">
          Create
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class NewAlbumDialog extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) visible!: boolean;

  // data
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
  // methods
}
</script>


<style lang="scss" scoped>
</style>
