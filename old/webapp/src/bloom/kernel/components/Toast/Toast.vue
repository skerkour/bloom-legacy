<template>
  <v-snackbar
      :timeout="timeout"
      :color="color"
      :bottom="y === 'bottom'"
      :top="y === 'top'"
      :left="x === 'left'"
      :right="x === 'right'"
      :auto-height = "autoHeight"
      :multi-line = "multiLine"
      :vertical = "vertical"
      v-model="active"
      class="application"
      @click="dismiss"
  >
    <v-icon
        dark
        left
        v-if="!!icon"
        class="mr-4"
    >
      {{ icon }}
    </v-icon>

    {{ message }}
    <v-btn
        color="white"
        flat
        @click="active = false"
      >
        Close
      </v-btn>
  </v-snackbar>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Toast extends Vue {
  @Prop({ type: String, default: 'right' }) x!: string;
  @Prop({ type: String, default: 'right' }) y!: string;
  @Prop({ type: String, default: 'info' }) color!: string;
  @Prop({ type: String }) icon?: string;
  @Prop({ type: String }) message?: string;
  @Prop({ type: Number, default: 6000 }) timeout!: number;
  @Prop({ type: Boolean, default: true }) dismissable!: boolean;
  @Prop({ type: Boolean, default: false }) autoHeight!: boolean;
  @Prop({ type: Boolean, default: false }) multiLine!: boolean;
  @Prop({ type: Boolean, default: false }) vertical!: boolean;

  active = false;

  mounted() {
    this.$nextTick(() => this.show());
  }

  show() {
    this.active = true;
  }

  close() {
    this.active = false;
  }

  dismiss() {
    if (this.dismissable) {
       this.close();
    }
  }

}
</script>
