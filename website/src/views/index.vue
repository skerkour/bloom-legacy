<template>
  <div>
    <!-- <blm-hello-world /> -->
    <v-container class="mt-3 text-center">
      <v-row row wrap justify-center>

        <v-col cols="12" sm="6" class="text-center">
          <h1 class="display-3 mb-5">Our mission: Empowering people</h1>

          <h4 class="subtitle-1 mt-5">Chat, Collaborate, Discover, Innovate.</h4>
          <h4 class="subtitle-1 mb-5">Take back control with
            <span class="blm-rounded-elegance">Bloom</span>
          </h4>

          <v-btn x-large color="primary" class="mt-5">
            <v-icon left>mdi-apple</v-icon>
            Downlaod for MacOS
          </v-btn>
        </v-col>

        <v-col cols="12" sm="6" class="text-center mb-5">
          <v-img alt="hero" src="/imgs/landing/hero.png" contain />
        </v-col>

      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import HelloWorld from '@/components/hello_world.vue';
import Footer from '@/components/footer.vue';


@Component({
  components: {
    'blm-hello-world': HelloWorld,
    'blm-footer': Footer,
  },
})
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>
