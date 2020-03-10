<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
       <v-col cols="12" class="mb-2">
        <h1 class="display-1 font-weight-medium">
          {{ $t('licensing.title') }}
        </h1>
      </v-col>

       <v-col cols="12" sm="10" md="8" class="mb-5">
        <p class="font-weight-regular text-left" v-html="$t('licensing.body')" />
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Licensing extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
</style>
