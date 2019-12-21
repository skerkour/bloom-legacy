<template>
  <v-container>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row>
      <v-col cols="12" sm="6" md="3">
        <v-card outlined>
          <v-card-title>Bloom</v-card-title>
          <v-card-text>v{{ version }}</v-card-text>
        </v-card>
      </v-col>


      <v-col cols="12" sm="6" md="3">
        <router-link to="/admin/users">
          <v-card outlined>
            <v-card-title>Users</v-card-title>
            <v-card-text>{{ users }}</v-card-text>
          </v-card>
        </router-link>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Dashboard extends Vue {
  // props
  // data
  error = '';
  version = '1.0.0';
  users = 9000;

  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
