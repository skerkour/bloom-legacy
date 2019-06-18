<template>
  <v-container >
    <v-layout row wrap>

      <v-flex xs12>
        <h1 class="display-1 font-weight-medium">
          Calendar
        </h1>
        <p class="font-18 font-weight-regular">
          Manage all your events in Calendar
        </p>
      </v-flex>

      <v-flex xs12 class="mt-4">
        <p class="font-18 font-weight-regular">
          The Bloom Calendar app helps you spend less time managing your schedule and more time enjoying it.
        </p>
      </v-flex>

    </v-layout>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}
</style>
