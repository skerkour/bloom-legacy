<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center">

        <h1 class="mt-5 mb-3 display-1 font-weight-medium">{{ $t('security.title') }}</h1>
        <p class="body-1 mb-5 font-weight-regular mb-5" v-html=" $t('security.description')" />

        <h2 class="mt-5 mb-3 display-1 font-weight-medium">{{ $t('security.issues_title') }}</h2>

        <p class="body-1 mb-5 font-weight-regular" v-html="$t('security.issues_description')"></p>

        <h2 class="mt-5 mb-3 display-1 font-weight-medium">{{ $t('security.hall_title') }}</h2>

        <p class="body-1 font-weight-regular">
          <ul>
            <li><a href="https://twitter.com/thejonmccoy" rel="noopener noreferrer">Jon McCoy</a></li>
            <li><a href="https://www.facebook.com/profile.php?id=100004793059302" rel="noopener noreferrer">Hassan Ahmed Hacker (Xen Lee)</a></li>
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
</style>
