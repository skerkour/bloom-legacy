<template>
  <div>

  <section id="main-section">
    <div>
      <img id="hero-image" src="/kernel/static/imgs/landing/hero_image.svg" />
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
                  <span class="rounded-elegance">Bloom</span>
                </h2>
                <h3 class="blm-primary-desc font-weight-regular mb-2">
                  Open internet services
                </h3>
                <!-- <p class="headline">
                  Bloom is on a mission to empower the world with open technologies, by the people, for the people
                </p> -->
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

    <section class="mb-5 mt-5 text-xs-center">
      <v-layout row wrap align-center justify-center>
        <v-flex xs8>
          <h2 class="blm-mission font-weight-regular">
            Bloom is on a mission to empower the world with open technologies
          </h2>
        </v-flex>
      </v-layout>
    </section>

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
          <p class="body-2">
            {{ service.description }}
          </p>
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
          <p>
            {{ service.description }}
          </p>
          <v-btn v-if="service.learn_more" :to="service.learn_more" flat color="primary">
            Learn more
          </v-btn>
          <v-btn to="/register" color="blue" dark>
            Try it Free
          </v-btn>
        </v-flex>
      </v-layout>
    </section>


        <v-card id="card-customers" class="elevation-0">
          <v-layout row wrap class="text-xs-center">
            <v-flex xs12>
              <h2 class="display-2 font-weight-regular">
                Built for humans, loved by developers
              </h2>
              <div class="mt-5 justify-content-center">
              <a target="_blank" rel="noopener" href="https://crisp.im/"
                class="ml-3 mr-3 mb-2 mt-2">
                  <img alt="crisp" title="crisp" src="/kernel/static/imgs/friends/crisp_blue.svg"
                    height="38">
                </a>
                <a target="_blank" rel="noopener" href="https://cryptio.co/"
                  class="ml-3 mr-3 mb-2 mt-2">
                  <img alt="styl" title="cryptio"
                    src="/kernel/static/imgs/friends/Cryptio_Logo_C_RGB.svg" height="42">
                </a>
                <a target="_blank" rel="noopener" href="http://www.42.fr/"
                  class="ml-3 mr-3 mb-2 mt-2">
                  <img alt="42" title="42" src="/kernel/static/imgs/friends/42.png" height="42">
                </a>
              </div>

              <div class="mt-2 mt-md-5 justify-content-center">

                <a target="_blank" rel="noopener" href="https://www.clevy.io/"
                  class="ml-3 mr-3 mb-2 mt-2">
                  <img alt="clevy" title="clevy" src="/kernel/static/imgs/friends/clevy.svg"
                    height="42">
                </a>
                <a target="_blank" rel="noopener" href="https://goosebump.com/"
                  class="ml-3 mr-3 mb-2 mt-2">
                  <img alt="lifeina" title="Goosebump"
                    src="/kernel/static/imgs/friends/goosebump.jpg" height="42">
                </a>
              </div>
            </v-flex>
          </v-layout>
        </v-card>

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
      description: 'The global cloud platform. Security scanner, serverless and much more...',
      learn_more: '/platform',
      logo: '/kernel/static/imgs/logos/platform.svg',
      name: 'Platform',
    },
    {
      description: 'A safe place for all your files. 30GB free.',
      learn_more: '/drive',
      logo: '/kernel/static/imgs/logos/drive.svg',
      name: 'Drive',
    },
    {
      description: 'Your cloud-based download manager. Download any file directly to your Drive.',
      learn_more: '/bitflow',
      logo: '/kernel/static/imgs/logos/bitflow.svg',
      name: 'Bitflow',
    },
    {
      description: 'Quickly capture what’s on your mind and get a reminder later at the right \
            place or time.',
      learn_more: '/notes',
      logo: '/kernel/static/imgs/logos/notes.svg',
      name: 'Notes',
    },
    {
      description: 'Keep your contacts organized and up to date.',
      learn_more: '/contacts',
      logo: '/kernel/static/imgs/logos/contacts.svg',
      name: 'Contacts',
    },
    {
      description: 'Never stop having fun!',
      learn_more: '/arcade',
      logo: '/kernel/static/imgs/logos/arcade.svg',
      name: 'Arcade',
    },
    {
      description: 'Never stop enjoying your music! Directly from your drive.',
      learn_more: '/music',
      logo: '/kernel/static/imgs/logos/music.svg',
      name: 'Music',
    },
    {
      description: 'Bloom Gallery is the home for all your photos and videos, \
              organized and beautifully presented.',
      learn_more: '/gallery',
      logo: '/kernel/static/imgs/logos/gallery.svg',
      name: 'Gallery',
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
}

.blm-primary-title {
  font-size: 42px;
}
.blm-primary-desc {
  font-size: 32px;
  display: block;
}
.blm-mission {
  font-size: 32px;
  color: #000;
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

#card-customers {
  margin-top: 75px;
}

@media (max-width: 960px) {
  #hero-image {
      left: 20vw;
      width: 60vw;
  }
  #service-platform {
    background: none;
    background-color: #e8e9fa;
  }
  #service-drive {
    background: none;
    background-color: #e3f9f0;
  }
  #service-bitflow {
    background: none;
    background-color: #ffe2b0;
  }
  #service-notes {
    background: none;
    background-color: #e3f9f0;
  }
  #service-contacts {
    background: none;
    background-color: #ffe2b0;
  }
}
</style>
