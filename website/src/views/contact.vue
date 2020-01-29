<template>
  <v-container class="mt-4 text-center">
    <v-row class="justify-center">
       <v-col cols="12" class="mb-2">
        <h1 class="display-1 font-weight-medium">
          {{ $t('contact.title') }}
        </h1>
      </v-col>

       <v-col cols="12" sm="10" md="8" class="mb-5">
        <p class="font-18 font-weight-regular text-left ">
          {{ $t('contact.intro') }}
          <ul>
            <li>
              <span v-html="$t('contact.chatbox')"></span>&nbsp;
            </li>
            <li>
              <span v-html="$t('contact.email')"></span>&nbsp;
              <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>
            </li>
            <li>
              <span v-html="$t('contact.gitlab')"></span>&nbsp;
              <a href="https://gitlab.com/bloom42/bloom/issues" target="_blank" rel="noopener">on GitLab</a>
            </li>
            <li>
              <span v-html="$t('contact.twitter')"></span>&nbsp;
              <a href="https://twitter.com/42bloom" target="_blank" rel="noopener">@42bloom</a>
            </li>
            <li>
              <span v-html="$t('contact.mastodon')"></span>&nbsp;
              <a href="https://mastodon.social/@42bloom" target="_blank" rel="noopener">@42bloom@mastodon.social</a>
            </li>
          </ul>
        </p>
      </v-col>

    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  created() {
    (window as any).$crisp.push(['do', 'chat:open']);
  }
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
</style>
