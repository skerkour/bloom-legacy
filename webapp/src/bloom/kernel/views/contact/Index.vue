<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>
      <v-flex xs12 class="text-xs-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Contact
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-5">
        <v-card class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular">
              You can contact the Bloom's team using the chatbox at the bottom right
              of our <a href="https://help.bloom.sh" target="_blank" rel="noopener">help website</a>
              , using the following email address:
              <a href="mailto:hello@bloom.sh">hello@bloom.sh</a> or on Twitter:
               <a href="https://twitter.com/bloom_cloud" target="_blank" rel="noopener">@bloom_cloud</a>
            </p>
          </v-card-text>
        </v-card>
      </v-flex>

    </v-layout>
  </v-container>

  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


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
