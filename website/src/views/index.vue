<template>
  <div>
    <!-- <blm-hello-world /> -->
    <v-container class="mt-3 text-center">
      <v-row row wrap justify-center>

        <v-col cols="12" md="6" class="text-center mb-5">
          <h1 class="display-3">Take back control of with
            <span class="blm-rounded-elegance">Bloom</span>
          </h1>
        </v-col>

        <v-col cols="12" md="6" class="text-center mb-5">
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
