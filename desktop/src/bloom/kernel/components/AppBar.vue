<template>
  <v-app-bar elevation="0" fixed app>

    <div class="blm-app-bar-left-spacer" />

    <!-- <span class="title ml-3 mr-5">Bloom</span> -->
    <span class="blm-appbar-title-primary ml-3 mr-5"><b>Bloom</b></span>
    <!-- <v-text-field
      outlined
      hide-details
      label="Seach"
      single-line
      prepend-inner-icon="mdi-magnify"
    /> -->

    <div class="flex-grow-1" />

    <v-tooltip bottom>
      <template v-slot:activator="{ on }">
        <v-btn icon @click="openHelp" v-on="on">
          <v-icon>mdi-help-circle-outline</v-icon>
        </v-btn>
      </template>
      <span>Help</span>
    </v-tooltip>
  </v-app-bar>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { shell } from 'electron';
import config from '@/config';


@Component
export default class BlmAppBar extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods

  openHelp() {
    shell.openExternal(config.HELP_URL);
  }
}
</script>


<style lang="scss" scoped>
.v-app-bar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1) !important;
  left: 0px !important;
}

.theme--light.v-app-bar.v-toolbar.v-sheet {
  background-color: #fff !important;
}

.theme--dark.v-app-bar.v-toolbar.v-sheet {
  background-color: #363636 !important;
}

.blm-appbar-title-primary {
  font-family: rounded_elegance;
  font-size: 1.6em;
  color: #424242;
}

.blm-app-bar-left-spacer {
  width: 80px;
}

.theme--dark {
  .v-app-bar {
    border-bottom: none !important;
  }

  .blm-appbar-title-primary {
    color: white;
  }

  // .v-app-bar.v-toolbar.v-sheet {
  //   background-color: #b3d4fc !important;
  // }
}
</style>
