<template>
  <div>

  <section id="main-section">
    <div>
      <img id="hero-image" src="/notes/static/imgs/landing/taking_notes.svg" />
    </div>
    <v-container>
      <v-layout row wrap>
        <v-flex xs12 class="text-xs-center">
        <v-hover>
        <v-card
        slot-scope="{ hover }"
        :class="`elevation-${hover ? 6 : 1}`"
        class="mb-5">
          <v-container>
            <v-layout row wrap>
              <v-flex xs12>
                <h2 class="blm-primary-title">
                  <span class="rounded-elegance">Bloom</span> Notes
                </h2>
                <p class="blm-primary-desc">
                  Quickly capture what’s on your mind
                </p>
                <v-btn to="/register" color="blue" dark>
                  Get started
                </v-btn>
                </v-flex>
              </v-layout>
            </v-container>
          </v-card>
        </v-hover>
        </v-flex>
      </v-layout>
    </v-container>
  </section>

<div style="height: 150px; overflow: hidden;" >
<svg viewBox="0 0 500 150" preserveAspectRatio="none" style="height: 100%; width: 100%;"><path d="M-0.00,50.02 C150.00,150.10 271.22,-50.03 500.00,50.02 L500.00,-0.00 L-0.00,-0.00 Z" style="stroke: none; fill: #75aee5;"></path></svg>
</div>


  <v-container class="pt-5 pb-5 text-xs-center text-md-left">

    <section
      v-for="(service, index) in services"
      class="text-xs-center pt-5 pb-5"
      :key="service.name">
      <v-layout row wrap align-center v-if="index % 2 === 0">
        <v-flex xs12 sm6 class="pt-2 pb-2">
          <v-img :src="service.logo" height="200" contain/>
        </v-flex>
        <v-flex xs12 sm6 class="pt-2 pb-2">
          <h2 class="display-1 font-weight-regular">
            {{ service.name }}
          </h2>
          <br />
          <p class="body-2" v-html="service.description"></p>
          <v-btn v-if="service.learn_more" :to="service.learn_more" flat color="primary">
            Learn more
          </v-btn>
          <v-btn to="/register" color="blue" dark>
            Try it Free
          </v-btn>
        </v-flex>
      </v-layout>
      <v-layout v-else row wrap align-center>
        <v-flex xs12 sm6 order-sm2 class="pt-2 pb-2">
          <v-img :src="service.logo" height="200" contain />
        </v-flex>
        <v-flex xs12 sm6 class="pt-2 pb-2">
          <h1 class="display-1 font-weight-regular">
            {{ service.name }}
          </h1>
          <br />
          <p class="body-2"  v-html="service.description"></p>
          <v-btn v-if="service.learn_more" :to="service.learn_more" flat color="primary">
            Learn more
          </v-btn>
          <v-btn to="/register" color="blue" dark>
            Try it Free
          </v-btn>
        </v-flex>
      </v-layout>
    </section>


    <section class="text-xs-center">
      <div class="mb-5 mt-5 elevation-0">
        <v-container>
          <v-layout row wrap>
            <v-flex xs12>
              <p class="blm-primary-desc">
                Get started with Bloom Notes for free
              </p>
              <v-btn to="/register" color="blue" dark>
                Go to Bloom Notes
              </v-btn>
              </v-flex>
            </v-layout>
          </v-container>
      </div>
    </section>

    </v-container>

  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Landing extends Vue {
  // props
  // data
  services = [
    {
      description: 'Bloom Notes helps you capture and prioritize ideas, projects, \
            so nothing falls through the cracks.',
      logo: '/notes/static/imgs/landing/organized.svg',
      name: 'Feel organized without the effort',
    },
    {
      description: 'Take note. It\'s easy to pull up your content from anywhere. Start on your \
        laptop then update notes on your phone. Bloom Notes works on any device or platform.',
      logo: '/notes/static/imgs/landing/devices.svg',
      name: 'Access from anywhere',
    },
    // {
    //   description: 'Quickly filter and search for notes. Find what you\'re looking \
    //     for even faster, and let Bloom notes do the remembering for you.',
    //   logo: '/notes/static/imgs/landing/search.svg',
    //   name: 'Find what you need, fast ',
    // },
  ];


  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}

.rounded-elegance {
  font-family: rounded_elegance;
  font-weight: bold;
}

.blm-primary-title {
  font-size: 42px;
  font-weight: normal;
}
.blm-primary-desc {
  font-size: 32px;
}

#hero-image {
  position: absolute;
  top: 1vh;
  height: 49vh;
  left: 30vw;
  width: 40vw;
}

#main-section {
  /* background: url(/imgs/landings/bloom/alt_wide_2.svg) no-repeat center fixed; */
  background: url(/kernel/static/imgs/landing/bg_1.svg) no-repeat center center;
  -webkit-background-size: fill; /* pour Chrome et Safari */
  -moz-background-size: fill; /* pour Firefox */
  -o-background-size: fill; /* pour Opera */
  background-size: fill; /* version standardisée */
  height: calc(70vh - 64px);
  padding-top: 52vh;
}
</style>
