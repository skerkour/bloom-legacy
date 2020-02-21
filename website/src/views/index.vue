<template>
  <v-container class="mt-3 text-center">
    <v-row class="justify-center align-center">

      <v-col cols="12" sm="6" class="text-center">
        <h1 :class="{'display-1': $vuetify.breakpoint.smAndDown,
        'display-3': $vuetify.breakpoint. mdAndUp, 'mb-5': true}">
          <!-- <vue-typer
            :text='pains'
            :repeat='Infinity'
            :shuffle='false'
            initial-action='typing'
            :pre-type-delay='60'
            :type-delay='20'
            :pre-erase-delay='7000'
            :erase-delay='250'
            erase-style='clear'
            :erase-on-complete='false'
            id="typewriter"
            @typed-char="onTypedChar"
          ></vue-typer> -->
          {{ pain }}
        </h1>

        <h4 :class="{'headline': $vuetify.breakpoint.smAndDown,
        'display-1': $vuetify.breakpoint. mdAndUp, 'mb-5': true}"
        v-html="$t('landing.subtitle2')"></h4>
        <h4 class="headline mb-4">{{ $t('landing.mission') }}</h4>

        <blm-download-btn />

        <blm-other-downloads-link />
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5">
        <!-- <img alt="hero" src="/static/imgs/landing/hero.png" height="480" /> -->
        <v-img alt="hero" src="/static/imgs/landing/hero.png" contain>
          <template v-slot:placeholder>
            <v-row
              class="fill-height ma-0"
              align="center"
              justify="center"
            >
              <v-progress-circular indeterminate color="grey lighten-2" />
            </v-row>
          </template>
        </v-img>
      </v-col>

    </v-row>

    <v-row class="text-center text--darken-1 grey--text" align="center" justify="center" >
      <v-col cols="12" sm="6" md="4" align="center" justify="center" class="pb-3">
        <h5 class="title mb-2">{{ $t('landing.product_hunt') }}</h5>
        <a href="https://www.producthunt.com/posts/bloom-05383d81-983e-4d4d-b22f-78dd91066cda" target="_blank" rel="noopener noreferer" class="blm-pointer">
          <v-img alt="Product Hunt" src="/static/imgs/landing/ph.svg" width="200px" contain>
            <template v-slot:placeholder>
              <v-row
                class="fill-height ma-0"
                align="center"
                justify="center"
              >
                <v-progress-circular indeterminate color="grey lighten-2" />
              </v-row>
            </template>
          </v-img>
        </a>
      </v-col>
      <v-col cols="12" sm="6" md="4" align="center" justify="center" class="pb-3">
        <h5 class="title mb-2">{{ $t('landing.hacker_news') }}</h5>
        <a href="https://news.ycombinator.com/item?id=20105567" target="_blank" rel="noopener noreferer" class="blm-pointer">
          <v-img alt="Hacker news" src="/static/imgs/landing/hn.svg" width="200px" >
            <template v-slot:placeholder>
              <v-row
                class="fill-height ma-0"
                align="center"
                justify="center"
              >
                <v-progress-circular indeterminate color="grey lighten-2" />
              </v-row>
            </template>
          </v-img>
        </a>
      </v-col>
    </v-row>

    <v-row class="mb-5 blm-features align-center">
      <v-col cols="12" sm="6" order="last" order-sm="first" class="text-center mb-5 mt-5">
        <img alt="secure" src="/static/imgs/landing/secure.svg" height="200" />
      </v-col>

      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          {{ $t('landing.feature1.title') }}
        </h3>
        <p class="mt-5" v-html="$t('landing.feature1.description')"></p>
      </v-col>
    </v-row>


    <v-row class="align-center">
      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          {{ $t('landing.feature3.title') }}
        </h3>
        <p class="mt-5" v-html="$t('landing.feature3.description')"></p>
      </v-col>

      <v-col cols="12" sm="6" class="text-center mb-5 mt-5">
        <img alt="groups" src="/static/imgs/landing/devices.svg" height="200" />
      </v-col>
    </v-row>

    <v-row class="mb-5 blm-features align-center">
      <v-col cols="12" sm="6" order="last" order-sm="first" class="text-center mb-5 mt-5">
        <img alt="secure" src="/static/imgs/landing/groups.svg" height="200" />
      </v-col>

      <v-col cols="12" sm="6">
        <h3 class="display-1 font-weight-regular">
          {{ $t('landing.feature2.title') }}
        </h3>
        <p class="mt-5" v-html="$t('landing.feature2.description')"></p>
      </v-col>
    </v-row>


    <v-row class="justify-center mb-5 mt-5">
      <v-col cols="12" class="text-center d-block d-sm-none">
        <blm-download-btn />
      </v-col>
      <v-col cols="12" class="text-center">
        <div class="d-none d-sm-inline-block">
          <blm-download-btn class="mr-5"/>
        </div>
        <v-btn to="/features" outlined x-large color="primary" class="ml-sm-5">
          {{ $t('landing.explore_features') }}
        </v-btn>
      </v-col>
    </v-row>

  </v-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import DownloadBtn from '@/components/coming_soon_btn.vue';
import OtherDownloadsLink from '@/components/other_downloads_link.vue';

@Component({
  components: {
    'blm-download-btn': DownloadBtn,
    'blm-other-downloads-link': OtherDownloadsLink,
  },
})
export default class Index extends Vue {
  // props
  // data
  countWord = 0;

  // computed
  get pain(): any {
    const pains = [this.$t('labels.pain1'), this.$t('labels.pain2'), this.$t('labels.pain3'),
      this.$t('labels.pain4')];
    return pains[Math.floor(Math.random() * pains.length)];
  }

  // lifecycle
  // watch
  // methods
  onTypedChar(typedChar: string, typedCharIndex: number) {
    if (typedCharIndex === 0) {
      this.countWord = 0;
      (document!.getElementById('typewriter')!.firstChild! as any).innerHTML = '';
    }
    const lessNodes = document.getElementById('typewriter')!.lastChild!.childNodes;
    if (typedChar === ' ' || lessNodes.length === 1) {
      const finalNodes = document.getElementById('typewriter')!.firstChild;
      const listNodes = finalNodes!.childNodes;

      const newNode = document.createElement('span');

      let x = this.countWord;
      const countNodes = listNodes.length;
      while (x < countNodes) {
        if ((listNodes[this.countWord] as any).innerHTML !== ' ') {
          newNode.insertAdjacentElement('beforeend', listNodes[this.countWord] as any);
        } else {
          this.countWord += 1;
        }
        // TODO: ADD LAST CHAR
        x += 1;
      }
      newNode.className = 'nowrap';
      (finalNodes as any).insertAdjacentElement('beforeend', newNode);

      this.countWord += 1;
    }
  }
}
</script>

<style  lang="scss">
span.nowrap{
  white-space: nowrap;
}

span.caret {
  display: none !important;
}
</style>

<style scoped lang="scss">
.blm-features {
  margin-top: 42px;
}
</style>
