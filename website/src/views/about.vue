<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">

      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Bloom
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center mb-4">
        <div class="elevation-0">
          <v-card-text>
            <p class="body-1 font-weight-regular">
              We believe that sharing and collaboration is the only way to build a
              sustainable future in a finite world.
              We create the open infrastructure to change all
              work from read-only to read-write. When everyone can contribute, users
              become contributors and we greatly improve the quality of global human life.
            </p>
          </v-card-text>
        </div>
      </v-col>


      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Mission
        </h1>
      </v-col>

      <v-flex xs12 sm10 md8 class="text-center mb-4">
        <div class="elevation-0">
          <v-card-text>
            <p class="body-1 font-weight-regular">
              Empowering the world with open technologies.
            </p>
          </v-card-text>
        </div>
      </v-flex>


      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Values
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center mb-4">
        <div class="elevation-0">
          <v-card-text>
            <p class="body-1 font-weight-regular">
              Cooperation, Acuity, Transparency, Simplicity (<b>CATS</b> 🐱).
            </p>
          </v-card-text>
        </div>
      </v-col>


      <v-col cols="12" class="text-center mb-5">
        <h1 class="display-1 font-weight-medium">
          Core team
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
            <p>Product design, France 🇫🇷</p>
          </a>
        </v-hover>
      </v-col>

      <v-col cols="12" sm="3" class="text-center mb-5">
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
      </v-col>


      <v-col cols="12" class="text-center mb-5">
        <h1 class="display-1 font-weight-medium" id="sponsors">
          Sponsors
        </h1>
        <p class="body-1 font-weight-regular">
          <a class="nav-link" href="/the-guide/organization/contributing.html#sponsoring">
            Want to sponsor Bloom?
          </a>
        </p>
      </v-col>

      <v-col cols="12" class="text-center mb-5">
        <div class="elevation-0">
          <v-card-text>
            <p class="body-1 font-weight-regular">
              -
            </p>
          </v-card-text>
        </div>
      </v-col>


      <v-col cols="12" class="text-center mb-3">
        <h1 class="display-1 font-weight-medium">
          Want to learn more?
        </h1>
      </v-col>

      <v-col cols="12" sm="10" md="8" class="text-center mb-4">
        <div class="elevation-0">
          <v-card-text>
            <a class="nav-link" href="/the-guide">
              Read <b>The Guide</b>
            </a>
          </v-card-text>
        </div>
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
