<template>
  <div>

  <section id="main-section">
    <div>
      <img id="hero-image" src="/bitflow/static/imgs/landing/downloading.svg" />
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
                  <span class="rounded-elegance">Bitflow</span>
                </h2>
                <p class="blm-primary-desc">
                  Download torrents to a secured cloud. Fast. Online.
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
        <v-card class="mb-5 mt-5 elevation-0">
        <v-container>
          <v-layout row wrap>
            <v-flex xs12>
              <p class="blm-primary-desc">
                Get started with Bitflow for free
              </p>
              <v-btn to="/register" color="blue" dark>
                Go to Bitflow
              </v-btn>
              </v-flex>
            </v-layout>
          </v-container>
        </v-card>
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
      description: 'Your files are downloaded directly to your Drive and can can be accessed from \
        any smartphone, tablet, or computer. So wherever you go, your downloaded files follow.',
      logo: '/bitflow/static/imgs/landing/devices.svg',
      name: 'Download anywhere, anytime',
    },
    {
      description: 'With Bitflow you don’t need to download any program, all you need is a \
      browser. Bitflow currently supports Torrent and HTTP(S) protocols.',
      logo: '/bitflow/static/imgs/landing/protocols.svg',
      name: 'No need to install a torrent client',
    },
    {
      description: 'It takes a long time to download large files with a residential bandwidth. \
        Not with Bitflow! We\'ll download your files in the blink of an eye!',
      logo: '/bitflow/static/imgs/landing/speed.svg',
      name: 'Download big files faster',
    },
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
