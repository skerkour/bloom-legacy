<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>
      <v-flex xs12 class="text-xs-center mb-2">
        <h1 class="display-1 font-weight-medium">
          Contact
        </h1>
      </v-flex>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-5">
        <div class="elevation-0">
          <v-card-text>
            <p class="font-18 font-weight-regular text-xs-left ">
              You can contact the Bloom team using one of the following way:
              <ul>
                <li>Using email: <a href="mailto:hello@bloom.sh">hello@bloom.sh</a></li>
                <li>Or by saying <i>Hi</i> on Twitter: <a href="https://twitter.com/42bloom" target="_blank" rel="noopener">@42bloom</a></li>
              </ul>
            </p>
          </v-card-text>
        </div>
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
