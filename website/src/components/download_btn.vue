<template>
  <v-btn x-large color="primary" :outlined="outlined">
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
        this.message = this.$t('download_btn.download_for', { platform: 'Linux' }).toString();
        this.icon = 'mdi-linux';
        break;
      case 'macos':
        this.message = this.$t('download_btn.download_for', { platform: 'macOS' }).toString();
        this.icon = 'mdi-apple';
        break;
      case 'windows':
        this.message = this.$t('download_btn.download_for', { platform: 'Windows' }).toString();
        this.icon = 'mdi-windows';
        break;
      case 'android':
        this.message = this.$t('download_btn.download_for', { platform: 'Android' }).toString();
        this.icon = 'mdi-android';
        break;
      case 'ios':
        this.message = this.$t('download_btn.download_for', { platform: 'iOS' }).toString();
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
