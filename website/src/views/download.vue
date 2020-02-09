<template>
  <v-container class="mt-3 text-center">

    <v-row class="justify-center">
      <v-col cols="12" sm="10" xl="8" class="text-center">
        <h2 :class="{'display-1': $vuetify.breakpoint.smAndDown,
        'display-3': $vuetify.breakpoint. mdAndUp}" class="mb-5">{{ pain }}</h2>
        <h1 :class="{'headline': $vuetify.breakpoint.smAndDown,
        'display-2': $vuetify.breakpoint. mdAndUp,
        'font-weight-bold': $vuetify.breakpoint.smAndDown}" class="mb-5">
          {{ $t('download.subtitle') }}
        </h1>
        <h3 class="headline mb-5 mt-5">
          {{ $t('download.subtitle2') }}
        </h3>
        <div class="mt-5 mb-5"></div>
        <blm-download-btn class="mt-5"/>
      </v-col>
    </v-row>

    <v-row class="justify-center blm-all-downloads mb-5">
      <v-col cols="12" sm="10" md="8" class="text-center">
        <h2 class="display-1">{{ $t('download.all') }}</h2>
      </v-col>
    </v-row>

    <v-row class="justify-center mb-5">
      <v-col cols="12" md="3" class="text-center">
        <v-card
          class="mx-auto pt-5 elevation-0"
          max-width="400"
        >
          <img
            class="white--text align-end mb-5 blm-inline-img"
            height="50px"
            src="/static/imgs/download/computer.svg"
            contain
          />
          <img
            class="white--text align-end mb-5 blm-inline-img"
            height="50px"
            src="/static/imgs/download/laptop.svg"
            contain
          />
          <v-card-text class="text-center justify-center">
            <h3 class="mb-3 title">{{ $t('download.computers_title') }}</h3>
            <p class="body-1">{{ $t('download.computers_description') }}</p>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" md="9">
        <v-row>

          <v-col cols="12" sm="6" md="3" class="text-center">
            <v-card
              class="mx-auto pt-5"
              max-width="400"
            >
              <img
                class="white--text align-end mb-5"
                height="100px"
                src="/static/imgs/download/apple.svg"
                contain
              />
              <v-divider />
              <v-card-actions class="text-center justify-center">
                <!-- <v-btn text>macOS</v-btn> -->
                <v-btn text disabled>{{ $t('download.coming_soon') }}</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>

          <v-col cols="12" sm="6" md="3" class="text-center">
            <v-card
              class="mx-auto pt-5"
              max-width="400"
            >
              <img
                class="white--text align-end mb-5"
                height="100px"
                src="/static/imgs/download/windows.svg"
                contain
              />
              <v-divider />
              <v-card-actions class="text-center justify-center">
                <!-- <v-btn text>Windows</v-btn> -->
                <v-btn text disabled>{{ $t('download.coming_soon') }}</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>

          <v-col cols="12" sm="6" md="3" class="text-center">
            <v-card
              class="mx-auto pt-5"
              max-width="400"
            >
              <img
                class="white--text align-end mb-5"
                height="100px"
                src="/static/imgs/download/linux.svg"
                contain
              />
              <v-divider />
              <v-card-actions class="text-center justify-center">
                <!-- <v-btn text>Linux</v-btn> -->
                <v-btn text disabled>{{ $t('download.coming_soon') }}</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>


          <v-col cols="12" sm="6" md="3" class="text-center">
            <a href="https://help.bloom.sh/en/article/how-to-install-the-command-line-client-b9350j"
              target="_blank" rel="noopener">
              <v-card
                class="mx-auto pt-5"
                max-width="400"
              >
                <img
                  class="white--text align-end mb-5"
                  height="100px"
                  src="/static/imgs/download/cli.svg"
                  contain
                />
                <v-divider />
                <v-card-actions class="text-center justify-center">
                  <v-btn text color="primary">{{ $t('download.cli') }}</v-btn>
                </v-card-actions>
              </v-card>
            </a>
          </v-col>

       </v-row>
      </v-col>

    </v-row>

    <v-row class="justify-center mt-5 mb-5">
      <v-divider />
    </v-row>


    <v-row class="justify-start mb-5 mt-5">
      <v-col cols="12" md="3" class="text-center">
        <v-card
          class="mx-auto pt-5 elevation-0"
          max-width="400"
        >
          <img
            class="white--text align-end mb-5 blm-inline-img"
            height="50px"
            src="/static/imgs/download/tablet.svg"
            contain
          />
          <img
            class="white--text align-end mb-5 blm-inline-img"
            height="50px"
            src="/static/imgs/download/phone.svg"
            contain
          />
          <v-card-text class="text-center justify-center">
            <h3 class="mb-3 title">{{ $t('download.mobile_title') }}</h3>
            <p class="body-1">{{ $t('download.mobile_description') }}</p>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" sm="6" md="3" class="text-center">
        <v-card
          class="mx-auto pt-5"
          max-width="400"
        >
          <img
            class="white--text align-end mb-5"
            height="100px"
            src="/static/imgs/download/android.svg"
            contain
          />
          <v-divider />
          <v-card-actions class="text-center justify-center">
            <!-- <v-btn text>Play Store</v-btn> -->
            <v-btn text disabled>{{ $t('download.coming_soon') }}</v-btn>
          </v-card-actions>
        </v-card>
      </v-col>

      <v-col cols="12" sm="6" md="3" class="text-center">
        <v-card
          class="mx-auto pt-5"
          max-width="400"
        >
          <img
            class="white--text align-end mb-5"
            height="100px"
            src="/static/imgs/download/apple.svg"
            contain
          />
          <v-divider />
          <v-card-actions class="text-center justify-center">
            <!-- <v-btn text>App Store</v-btn> -->
            <v-btn text disabled>{{ $t('download.coming_soon') }}</v-btn>
          </v-card-actions>
        </v-card>
      </v-col>

    </v-row>

  <blm-download-dialog-cli
    :visible="isCliDialogVisible"
    @closed="cliDialogClosed"
  />
  </v-container>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import BlmDownloadBtn from '@/components/coming_soon_btn.vue';


@Component({
  components: {
    BlmDownloadBtn,
  },
})
export default class Download extends Vue {
  // props
  // data
  // computed
  get pain(): any {
    const pains = [this.$t('labels.pain1'), this.$t('labels.pain2'), this.$t('labels.pain3'),
      this.$t('labels.pain4')];
    return pains[Math.floor(Math.random() * pains.length)];
  }

  // lifecycle
  // watch
  // methods
  openCliInstructions() {

  }
}
</script>

<style scoped land="scss">
.blm-all-downloads {
  margin-top: 100px;
}

.v-card {
  border-radius: 10px;
}

.blm-inline-img {
  display: inline-block;
  margin-right: 10px;
}
</style>
