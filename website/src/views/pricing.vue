<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="10" sm="8" md="10" class="text-center mt-5">
        <h1 class="display-3 mb-5">Pricing</h1>
        <v-row class="justify-center">
          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="display-1 justify-center">Free</v-card-title>
              <v-card-text>
                blah blah blah
                <v-divider class="mt-3 mb-3" />
                <p class="title">0€ / m</p>
              </v-card-text>
            </v-card>
          </v-col>

          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="display-1 justify-center">Basic</v-card-title>
              <v-card-text>
                blah blah blah
                <v-divider class="mt-3 mb-3" />
                <p class="title">4.99€ / m</p>
              </v-card-text>
            </v-card>
          </v-col>

          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="display-1 justify-center">Pro</v-card-title>
              <v-card-text>
                blah blah blah
                <v-divider class="mt-3 mb-3" />
                <p class="title">9.99€ / m</p>
              </v-card-text>
            </v-card>
          </v-col>

          <v-col cols="12" md="3" class="text-center mt-5">
            <v-card class="mx-auto" outlined>
              <v-card-title class="display-1 justify-center">Ultra</v-card-title>
              <v-card-text>
                blah blah blah
                <v-divider class="mt-3 mb-3" />
                <p class="title">19.99€ / m</p>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center">
        <blm-download-btn />
        <blm-other-downloads-link />
      </v-col>
    </v-row>

    <v-row class="justify-center align-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">Save big with Bloom</h1>
      </v-col>

      <v-col cols="10" sm="8" md="5" class="text-center mt-5">
        <v-card class="mx-auto" outlined>
          <v-card-title class="display-1 justify-center">Bloom</v-card-title>
          <v-card-text class="text-left">
            <v-row class="justify-center">
              <v-col cols="12" sm="10" md="8">
                <ul>
                  <li><v-icon color="success">mdi-check</v-icon> Notes</li>
                  <li><v-icon color="success">mdi-check</v-icon> Drive</li>
                  <li><v-icon color="success">mdi-check</v-icon> Calendar</li>
                  <li><v-icon color="success">mdi-check</v-icon> Torrents</li>
                  <li>...And much <a href="https://gitlab.com/bloom42/wiki/-/wikis/organization/roadmap" target="_blank" rel="noopener noreferer">more coming</a></li>
                </ul>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" md="1" class="text-center mt-5">
        <h3 class="display-1">VS</h3>
      </v-col>

      <v-col cols="10" sm="8" md="5" class="text-center mt-5">
        <v-card class="mx-auto" outlined>
          <v-card-title  class="display-1 justify-center">Others</v-card-title>
          <v-card-text class="text-left">
            <v-row class="justify-center">
              <v-col cols="12" sm="10" md="8">
                <ul>
                  <li><v-icon color="primary">mdi-dropbox</v-icon> Dropbox 9.99€</li>
                  <li><v-icon color="green">mdi-evernote</v-icon> Evernote 6.99€</li>
                  <li><v-icon color="success">mdi-anchor</v-icon> Bitport 8€</li>
                  <v-divider class="mt-5 mb-5" />
                  <li><b>24.98€ / m</b></li>
                </ul>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-row class="justify-center">
      <v-col cols="12" sm="10" md="8" class="text-center mt-5">
        <h1 class="display-3 mb-5">Having questions?</h1>
        <div class="mt-5 pt-5">
          <router-link outlined to="/faq" >Explore the FAQ</router-link>
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
  // computed
  // lifecycle
  // watch
  // methods
}
</script>

<style scoped lang="scss">
ul {
  list-style-type: none;
}

li {
  margin-top: 9px;
}

.v-card__text {
  font-size: 20px !important;
}

.v-application p {
  margin-bottom: 0px;
}
</style>
