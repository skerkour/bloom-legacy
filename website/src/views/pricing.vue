<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="12" class="text-center">
        <h1 class="display-3 mb-5">{{ $t('pricing.title') }}</h1>
      </v-col>
    </v-row>

    <v-row justify="space-around" align="stretch">
      <v-col cols="12" sm="6" md="3" class="text-center mt-5" align-self="stretch"
        v-for="plan in plans" :key="plan.name">
        <v-hover v-slot:default="{ hover }">
          <v-card class="mx-auto blm-pricing-card" outlined :elevation="hover ? 4 : 0"
            :class="{ 'on-hover': hover }">
            <v-card-title class="display-1 justify-center">{{ plan.name }}</v-card-title>
            <div class="v-card--plan__price pa-5 col col-12" v-if="plan.price === 0">
              <div class="d-inline-block">
                <span class="display-3 font-weight-bold">Free</span>
              </div>
            </div>
            <div class="v-card--plan__price pa-5 col col-12" v-else>
              <div class="d-inline-block">
                <span class="display-3 font-weight-bold">{{ plan.price }}€</span>
              </div>
              <span class="caption"> /mo </span>
            </div>
            <v-card-text class="blm-pricing-card-text" v-html="plan.description">
            </v-card-text>
          </v-card>
        </v-hover>
      </v-col>
    </v-row>

    <v-row class="justify-center mt-5">
      <v-col cols="12" sm="10" md="8" class="text-center">
        <blm-download-btn />
        <blm-other-downloads-link />
      </v-col>
    </v-row>

    <v-row class="justify-center" align="stretch">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">{{ $t('pricing.title2') }}</h1>
      </v-col>

      <v-col cols="10" sm="8" md="5" class="text-center mt-5" align-self="stretch">
        <v-card class="mx-auto blm-pricing-card" outlined>
          <v-card-title class="display-1 justify-center">Bloom</v-card-title>
          <v-card-text class="text-left blm-pricing-card-text" >
            <v-row class="justify-center">
              <v-col cols="12" sm="10" md="8">
                <ul>
                  <li><v-icon color="success">mdi-check</v-icon> Drive</li>
                  <li><v-icon color="success">mdi-check</v-icon> Notes</li>
                  <li><v-icon color="success">mdi-check</v-icon> <b>Encryption</b></li>
                  <li><v-icon color="success">mdi-check</v-icon> Torrents</li>
                  <li><v-icon color="success">mdi-check</v-icon> Calendar</li>
                  <li><v-icon color="success">mdi-check</v-icon> Contacts</li>
                  <li><v-icon color="success">mdi-check</v-icon> Gallery</li>
                  <li><v-icon color="success">mdi-check</v-icon> Music</li>
                  <li><v-icon color="success">mdi-check</v-icon> <b>Open Source</b></li>
                  <li><v-icon color="success">mdi-check</v-icon> <b>Private by design</b></li>
                  <li><v-icon color="success">mdi-check</v-icon> <b>Ethical by design</b></li>
                  <li>... <router-link to="/features">And much more</router-link></li>
                </ul>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" md="1" class="text-center mt-5" align-self="center">
        <h3 class="display-1">VS</h3>
      </v-col>

      <v-col cols="10" sm="8" md="5" class="text-center mt-5" align-self="stretch">
        <v-card class="mx-auto blm-pricing-card" outlined>
          <v-card-title  class="display-1 justify-center">{{ $t('pricing.others') }}</v-card-title>
          <v-card-text class="text-left blm-pricing-card-text">
            <v-row justify="center" align="center">
              <v-col cols="12" sm="10" md="8" align-self="center">
                <ul>
                  <li><v-icon color="primary">mdi-dropbox</v-icon> Dropbox <b>9.99€</b></li>
                  <li><v-icon color="green">mdi-evernote</v-icon> Evernote <b>6.99€</b></li>
                  <li><v-icon color="success">mdi-anchor</v-icon> Bitport <b>8€</b></li>
                  <li><v-icon color="red">mdi-google</v-icon> Google Suite <b>5.20€</b></li>
                  <v-divider class="mt-5 mb-5" />
                  <li><b>29.18€ / mo</b></li>
                  <li><v-icon color="error">mdi-close</v-icon> Privacy</li>
                  <li><v-icon color="error">mdi-close</v-icon> Ethical technology</li>
                </ul>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">{{ $t('pricing.title3') }}</h1>
        <div class="mt-5 pt-5">
          <router-link outlined to="/faq">{{ $t('pricing.explore_faq') }}</router-link>
        </div>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center">
        <blm-download-btn />
        <blm-other-downloads-link />
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
export default class Pricing extends Vue {
  // props
  // data
  plans = [
    {
      name: 'Free',
      price: 0,
      description: `
      <ul>
        <li>1GB storage</li>
        <li>Notes</li>
        <li>QR Codes</li>
        <li>Music</li>
        <li>Gallery</li>
        <li>1 Bitflow download / day</li>
        <li>Community Support</li>
      </ul>
      `,
    },
    {
      name: 'Basic',
      price: 7,
      description: `
      <ul>
        <li><b>All features from Free</b></li>
        <li class="text-center bold"><b>+</b></li>

        <li>100GB storage</li>
        <li>Calendar</li>
        <li>Contacts</li>
        <li>1 parallel Bitflow download</li>
        <li>Friendly Support</li>
      </ul>
      `,
    },
    {
      name: 'Pro',
      price: 14,
      description: `
      <ul>
        <li><b>All features from Basic</b></li>
        <li class="text-center bold"><b>+</b></li>

        <li>400GB storage</li>
        <li>2 parallel Bitflow downloads</li>
      </ul>
      `,
    },
    {
      name: 'Ultra',
      price: 28,
      description: `
        <ul>
          <li><b>All features from Pro</b></li>
          <li class="text-center bold"><b>+</b></li>

          <li>1TB storage</li>
          <li>4 parallel Bitflow downloads</li>
          <li>Priority Support</li>
        </ul>
      `,
    },
  ]
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss">
.blm-pricing-card-text ul {
  list-style-type: none;
}

.blm-pricing-card-text ul li {
  margin-top: 9px;
}
</style>

<style scoped lang="scss">
.v-card__text {
  font-size: 20px !important;
}

.v-application p {
  margin-bottom: 0px;
}

.blm-pricing-card {
  height: 100%;
}
</style>
