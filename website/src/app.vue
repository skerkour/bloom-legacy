<template>
  <v-app>
    <blm-app-bar />
    <v-content>
      <router-view/>
    </v-content>
    <blm-footer />
  </v-app>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import AppBar from '@/components/app_bar.vue';
import Footer from '@/components/footer.vue';


@Component({
  components: {
    'blm-app-bar': AppBar,
    'blm-footer': Footer,
  },
})
export default class App extends Vue {
}
</script>


<style lang="scss">
.v-alert a {
  color: #FAA200 !important;
}

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
  src: url("/static/fonts/rounded_elegance.ttf") format("truetype");
}

body {
  font-size: 19px;
  @media (max-width: 768px) {
    font-size: 17px;
  }
}

a, a * {
  text-decoration: none !important;
  // color: inherit !important;
}

.blm-rounded-elegance {
  font-family: rounded_elegance !important;
  font-size: unset;
}

.v-btn {
  border-radius: 4px;
  text-transform: none; /* overwrite default 'uppercase' */
}

// not sure why it does not work in block below
.theme--dark.v-application {
  background-color: #303030;
}
.theme--dark {
  .v-toolbar, .v-table, .v-footer, .v-tabs__bar, .v-application {
    background-color: #303030;
  }

  pre {
    background-color: #212121;
  }
}

// not sure why it does not work in block below
.theme--light.v-application {
  background-color: #fefefe;
}
.theme--light {
  .v-toolbar, .v-table, .v-footer, .v-tabs__bar, .v-application {
    background-color: #fefefe;
  }

  pre {
    background-color: #f0f0f0;
  }
}

.blm-pointer {
  cursor: pointer;
}
</style>
