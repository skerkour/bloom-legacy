<template>
  <v-container class="mt-3 text-center">
    <v-row row wrap justify-center>

      <v-col cols="12" class="text-center mb-5">
        <v-img alt="404" src="/static/imgs/404/404.svg" id="img-404" contain  height="400"/>
      </v-col>

      <v-col cols="12" class="text-center">
        <h1 class="display-2 font-weight-medium">
          {{ $t('p404.not_found') }}
        </h1>
      </v-col>

      <v-col cols="12" class="text-center mt-4 mb-4">
        <v-btn to="/" color="primary">{{ $t('p404.back_home') }}</v-btn>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component({})
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
#img-404 {
  max-height: 50vh;
}

a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}

.font-5 {
  font-size: 15px;
}
</style>
