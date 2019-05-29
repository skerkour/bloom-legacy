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
                Statu quo whether we are talking about software, science, education, research...<br/>
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
                  <a :href="project.homepage" v-if="project.homepage">
                    Homepage
                  </a>
                  <span v-if="project.homepage && project.repository"> - </span>
                  <a :href="project.repository" v-if="project.repository">
                    Git Repository
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
      homepage: '/',
      img: '/kernel/static/imgs/logos/bloom_256.png',
      name: 'Bloom',
      repository: 'https://github.com/bloom42/bloom',
    },
    {
      description: 'The cloud native security scanner ☁️',
      homepage: '/phaser',
      img: '/kernel/static/imgs/logos/phaser_256.png',
      name: 'Phaser',
      repository: 'https://github.com/bloom42/phaser',
    },
    {
      description: 'A simple piece of software that makes downloading ﬁles fast ⚡',
      homepage: '/bitflow',
      img: '/kernel/static/imgs/logos/bitflow_256.png',
      name: 'Bitflow',
      repository: 'https://github.com/bloom42/bitflow',
    },
    {
      description: 'Automated software delivery as fast and easy as possible',
      homepage: '/rocket',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Rocket',
      repository: 'https://github.com/bloom42/rocket',
    },
    {
      description: 'Simple and beautiful Analytics',
      homepage: '/signal',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Signal',
      repository: 'https://github.com/bloom42/signal',
    },
    {
      description: 'The Simple And NEeded configuration format',
      homepage: '/sane',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'SANE',
      repository: 'https://github.com/bloom42/sane',
    },
    // {
    //   description: 'Automated Code Review for gO',
    //   homepage: '/acro',
    //   img: '/kernel/static/imgs/img_square_3.svg',
    //   name: 'Acro',
    //   repository: 'https://github.com/bloom42/acro',
    // },
    {
      description: 'Bloom\'s open source website',
      homepage: 'https://opensource.bloom.sh/',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Open Source',
      repository: 'https://github.com/bloom42/opensource',
    },
    {
      description: 'Correct commonly misspelled English words in source files 📖',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'misspell',
      repository: 'https://github.com/bloom42/misspell',
    },
    {
      description: 'Automated Hacker News curation',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Newsbot',
      repository: 'https://github.com/bloom42/newsbot',
    },
    {
      description: 'RipZap - The fastest structured, leveled JSON logger for Go ⚡️. ' +
        'Dependency free.',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'rz-go',
      repository: 'https://github.com/bloom42/rz-go',
    },
    {
      description: 'Fast, elegant, structured, pluggable and isomorphic logging for Javascript',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Astro-js',
      repository: 'https://github.com/bloom42/astro-js',
    },
    {
      description: 'Fast and configurable filesystem (file and directory names) linter',
      img: '/kernel/static/imgs/img_square_3.svg',
      name: 'Flint',
      repository: 'https://github.com/bloom42/flint',
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
