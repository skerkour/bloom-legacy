<template>
  <div>
  <v-container>
    <v-layout row>
      <v-flex xs12>
          <v-container fluid grid-list-md >
            <v-layout row wrap >

            <v-flex xs12 sm10 offset-sm1 md8 offset-md2 class="text-xs-center mb-3">
              <h1 class="display-1 font-weight-medium mb-4">
                Open Source & Free software
              </h1>
              <p class="text-xs-left text--darken-1 font-18">
                Openness is the DNA of Bloom. We are strongly committed to promote and improve the
                Status quo whether we are talking about software, science, education, research...<br/>
                <b>All our production</b> (software, design...) is and will ever be <b>free</b> (as in Freedom).
                You can find more details about our licensing policy on our
                <router-link to="/licensing">Licensing page</router-link>.
              </p>
            </v-flex>

              <v-flex v-for="project in projects" xs6 sm4 :key="project.name" class="mt-5 text-xs-center">
                <v-img :src="project.img" height="150px" contain class="app-icon"></v-img>
                <p class="headline app-name mt-3">{{ project.name }}</p>
                <p class="subheading mt-3">{{ project.description }}</p>
                <p class="subheading">
                  <!-- <a :href="project.homepage" v-if="project.homepage">
                    Homepage
                  </a> -->
                  <!-- <span v-if="project.homepage && project.repository"> - </span> -->
                  <a :href="project.homepage" v-if="project.homepage">
                    Homepage
                  </a>
                </p>
              </v-flex>

            </v-layout>
          </v-container>
      </v-flex>
    </v-layout>
  </v-container>
  <blm-footer />
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Projects extends Vue {
  // props
  // data
  projects = [
    {
      description: 'The current website',
      homepage: 'https://gitlab.com/bloom42/bloom',
      img: '/kernel/static/imgs/logos/bloom_256.png',
      name: 'Bloom',
    },
    {
      description: 'The cloud native security scanner ☁️',
      homepage: 'https://gitlab.com/bloom42/phaser',
      img: '/phaser/static/imgs/phaser_rounded_256.png',
      name: 'Phaser',
    },
    {
      description: 'A simple piece of software that makes downloading ﬁles fast ⚡',
      homepage: 'https://gitlab.com/bloom42/bitflow',
      img: '/kernel/static/imgs/logos/bitflow_256.png',
      name: 'Bitflow',
    },
    {
      description: 'The Simple And NEeded configuration format',
      homepage: 'https://gitlab.com/bloom42/sane',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'SANE',
    },
    {
      description: 'SANE parser for Rust',
      homepage: 'https://gitlab.com/bloom42/sane-rs',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'SANE-rs',
    },
    {
      description: 'SANE parser for Go',
      homepage: 'https://gitlab.com/bloom42/sane-go',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'SANE-go',
    },
    {
      description: 'Vim syntax for the SANE configuration format',
      homepage: 'https://gitlab.com/bloom42/sane-vim',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'SANE-vim',
    },
    {
      description: 'Correct commonly misspelled English words in source files 📖',
      homepage: 'https://gitlab.com/bloom42/tools/misspell',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'misspell',
    },
    {
      description: 'Fast, elegant, structured, pluggable and isomorphic logging for Javascript',
      homepage: 'https://gitlab.com/bloom42/astro-js',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Astro-js',
    },
    {
      description: 'Fast and configurable filesystem (file and directory names) linter',
      homepage: 'https://gitlab.com/bloom42/tools/flint',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Flint',
    },
    {
      description: 'RipZap - The fastest structured, leveled JSON logger for Go ⚡️. ' +
        'Dependency free.',
      homepage: 'https://github.com/z0mbie42/rz-go',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'rz-go',
    },
    {
      description: 'Automated Hacker News curation',
      homepage: 'https://github.com/z0mbie42/newsbot',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Newsbot',
    },
    {
      description: 'Automated software delivery as fast and easy as possible',
      homepage: 'https://github.com/z0mbie42/rocket',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Rocket',
    },
    {
      description: 'Simple and beautiful Analytics',
      homepage: 'https://github.com/z0mbie42/signal/',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Signal',
    },
  ];


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

.font-5 {
  font-size: 15px;
}
</style>
