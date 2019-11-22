<template>
  <v-container class="mt-4 text-center">
    <v-row>
       <v-col cols="12" class="text-center mb-2">
        <h1 class="display-1 font-weight-medium">
          Contact
        </h1>
      </v-col>

       <v-col cols="12" sm10 md8 class="text-center mb-5">
        <p class="font-18 font-weight-regular text-left ">
          You can contact the Bloom team using one of the following way:
          <ul>
            <li>Using email: <a href="mailto:hello@bloom.sh">hello@bloom.sh</a></li>
            <li>Or by saying <i>Hi</i> on Twitter: <a href="https://twitter.com/42bloom" target="_blank" rel="noopener">@42bloom</a></li>
          </ul>
        </p>
      </v-col>

    </v-row>
  </v-container>
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
