<template>
  <v-app>
    <component :is="layout">
      <router-view/>
    </component>
  </v-app>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

const DEFAULT_LAYOUT: string = 'default';

@Component
export default class App extends Vue {
  get layout(): string {
    if (this.$store.state.session && this.$route.meta.auth) {
      return `blm-layout-${this.$route.meta.auth.layout || DEFAULT_LAYOUT}`;
    }
    return `blm-layout-${this.$route.meta.layout || DEFAULT_LAYOUT}`;
  }
}
</script>

<style lang="scss">
// Global vars
$grey: #2c3e50;
$white: #fff;
$red: #F44336;
$blue: #2196F3;

$success: #67C23A;
$info: #409EFF;
$warning: #E6A23C;
$danger: #F56C6C;

@font-face {
	font-family: "rounded_elegance";
	src: url("/kernel/static/fonts/rounded_elegance.ttf") format("truetype");
}

#app {
  /* font-family: 'Avenir', Helvetica, Arial, sans-serif; */
  /* -webkit-font-smoothing: antialiased; */
  /* -moz-osx-font-smoothing: grayscale; */
  font-family: proxima-nova,-apple-system,BlinkMacSystemFont,avenir next,avenir,
  helvetica neue,helvetica,ubuntu,roboto,noto,segoe ui,arial,sans-serif;
  color: #2c3e50;
  background-color: $white;
}

.v-btn {
  border-radius: 4px;
  text-transform: none; /* overwrite default 'uppercase' */
}

.v-btn--floating, .v-btn--icon::before {
  border-radius: 50%;
}


.pointer {
  cursor: pointer;
}

/* fix outline text fields */
.v-text-field--box input, .v-text-field--outline input {
  margin-top: 15px;
}
.v-text-field--box .v-label, .v-text-field--outline .v-label {
  top: 8px;
}
.v-text-field--box .v-input__slot, .v-text-field--outline .v-input__slot {
  min-height: 32px;
}

.v-text-field--outline .v-text-field__prefix,
.v-text-field--outline.v-input--is-dirty .v-text-field__prefix,
.v-text-field--outline.v-input--is-focused .v-text-field__prefix {
  margin-top: 9px;
}

.elevation-0 {
  -webkit-box-shadow: none !important;
	-moz-box-shadow: none !important;
	box-shadow: none !important;
}
</style>
