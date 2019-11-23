<template>
  <v-btn x-large color="primary" class="mt-5" :outlined="outlined">
    <v-icon left v-text="icon" v-if="icon !== ''"/>
    {{ message }}
  </v-btn>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { detectPlatform } from '@/utils/platform';

@Component
export default class AppBar extends Vue {
  // props
  @Prop({ type: String }) platform?: string;
  @Prop({ type: Boolean }) outlined?: boolean;


  // data
  message: string = '';
  icon: string = ''

  // computed
  created() {
    let { platform } = this;
    if (!this.platform) {
      platform = detectPlatform();
    }
    switch (platform) {
      case 'linux':
        this.message = 'Download for Linux';
        this.icon = 'mdi-linux';
        break;
      case 'macos':
        this.message = 'Download for macOS';
        this.icon = 'mdi-apple';
        break;
      case 'windows':
        this.message = 'Download for Windows';
        this.icon = 'mdi-windows';
        break;
      case 'android':
        this.message = 'Download for Android';
        this.icon = 'mdi-android';
        break;
      case 'ios':
        this.message = 'Download for iOS';
        this.icon = 'mdi-apple';
        break;
      default:
        this.message = 'Download Bloom';
    }
  }
  // lifecycle
  // watch
  // methods
  goto(path: string) {
    this.$router.push({ path });
  }
}
</script>

<style>
</style>
