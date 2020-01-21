<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">

      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Bloom
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center mb-4">
        <p class="font-weight-regular">
          {{ $t('about.believe') }}
        </p>
      </v-col>


      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          {{ $t('about.mission.title') }}
        </h1>
      </v-col>

      <v-flex xs12 sm10 md8 class="text-center mb-4">
        <p class="font-weight-regular">{{ $t('about.mission.description') }}</p>
      </v-flex>


      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          {{ $t('about.values.title') }}
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center mb-4">
        <p class="font-weight-regular" v-html="$t('about.values.description')"></p>
      </v-col>


      <v-col cols="12" class="text-center mb-5">
        <h1 class="display-1 font-weight-medium">
          {{ $t('about.team.title') }}
        </h1>
      </v-col>

      <v-col cols="12" sm="3" class="text-center">
        <v-hover>
          <a slot-scope="{ hover }" href="https://kerkour.fr" target="_blank" rel="noopener">
            <v-avatar size="100px" color="grey lighten-4">
              <img src="/static/imgs/about/sylvain.jpg" alt="Sylvain Kerkour"
              :class="`elevation-${hover ? hoverElevation : 0}`">
            </v-avatar>
            <h3 class="mt-3">
              Sylvain Kerkour
            </h3>
            <p><!-- {{ $t('about.sylvain_title') }}, -->France 🇫🇷</p>
          </a>
        </v-hover>
      </v-col>

      <!-- <v-col cols="12" sm="3" class="text-center mb-5">
        <v-hover>
          <a slot-scope="{ hover }" href="https://www.linkedin.com/in/marina-da-silva-b289b4159"
          target="_blank" rel="noopener">
            <v-avatar size="100px" color="grey lighten-4">
              <img src="/static/imgs/about/marina.jpg" alt="Marina Da Silva"
              :class="`elevation-${hover ? hoverElevation : 0}`">
            </v-avatar>
            <h3 class="mt-3">
              Marina Da Silva
            </h3>
            <p>Head of operations, France 🇫🇷</p>
          </a>
        </v-hover>
      </v-col> -->


      <!-- <v-col cols="12" class="text-center mb-5">
        <h1 class="display-1 font-weight-medium mb-5" id="sponsors">
          {{ $t('about.sponsors.title') }}
        </h1>
        <p class="font-weight-regular">
          <a class="nav-link" href="/the-guide/organization/contributing.html#sponsoring">
            {{ $t('about.sponsors.want') }}
          </a>
        </p>
      </v-col> -->

      <!-- <v-col cols="12" class="text-center mb-5">
        <p class="font-weight-regular">-</p>
      </v-col> -->


      <v-col cols="12" class="text-center">
        <h1 class="display-1 font-weight-medium">
          {{ $t('about.learn_more.title') }}
        </h1>
      </v-col>

      <v-col cols="12" class="text-center mb-4">
        <p v-html="$t('about.learn_more.blog')" />
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class About extends Vue {
  // props
  // data
  hoverElevation = 4;
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
</style>
