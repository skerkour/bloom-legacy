<template>
  <!-- eslint-disable -->
  <v-footer height="auto" class="text-left">
    <v-container>
      <v-row class="text-center text-sm-left">
        <v-col cols="12" sm="4" class="pa-4">
          <h5 class="title pb-3">{{ $t('footer.organization.title') }}</h5>
          <ul>
            <li><router-link class="nav-link" to="/about">{{ $t('footer.organization.about') }}</router-link></li>
            <li><router-link class="nav-link" to="/contact">{{ $t('footer.organization.contact') }}</router-link></li>
            <li><router-link class="nav-link" to="/security">{{ $t('footer.organization.security') }}</router-link></li>
            <li><router-link class="nav-link" to="/terms">{{ $t('footer.organization.terms') }}</router-link></li>
            <li><router-link class="nav-link" to="/privacy">{{ $t('footer.organization.privacy') }}</router-link></li>
            <li><a class="nav-link" href="/the-guide">The Guide</a></li>
          </ul>
        </v-col>

        <v-col cols="12" sm="4" class="pa-4">
          <h5 class="title pb-3">{{ $t('footer.product.title') }}</h5>
          <ul>
            <li><router-link to="/help">{{ $t('footer.product.help') }}</router-link></li>
            <li><router-link class="nav-link" to="/download">{{ $t('footer.product.download') }}</router-link></li>
            <li><a class="nav-link" href="/the-guide/organization/contributing.html">{{ $t('footer.product.contribute') }}</a></li>
            <li><a class="nav-link" href="/the-guide/projects/bloom/index.html">{{ $t('footer.product.documentation') }}</a></li>
          </ul>
        </v-col>

        <v-col cols="12" sm="4" class="pa-4">
          <h5 class="title pb-3">{{ $t('footer.community.title') }}</h5>
          <ul>
            <li><a class="nav-link" href="/blog">Blog</a></li>
            <li><a class="nav-link" href="https://gitlab.com/bloom42" target="_blank" rel="noopener">Open Source</a></li>
            <li><a class="nav-link" href="https://twitter.com/42bloom" target="_blank" rel="noopener">Twitter</a></li>
            <li><a class="nav-link" href="https://discord.gg/HmDQDGv" target="_blank" rel="noopener">Discord Chat</a></li>
            <li><a class="nav-link" href="/the-guide/organization/contributing.html#sponsoring">{{ $t('footer.community.sponsoring') }}</a></li>
          </ul>
        </v-col>

        <v-col cols="12" class="blm-footer-year-col">
          <p class="text-center blm-footer-year">
            <!-- <span><a href="mailto:hello@bloom.sh">hello@bloom.sh</a></span><br/><br/> -->
            <span>&copy; 2018 - {{ new Date().getFullYear() }} Bloom</span>
          </p>
        </v-col>

      </v-row>
    </v-container>
  </v-footer>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Footer extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
.theme--dark.v-footer {
  a {
    color: white;
  }
}

.theme--light.v-footer {
  a {
    color: black;
  }
}

a {
  text-decoration: none;
}

ul li {
  margin-bottom: 10px;
}

ul {
  list-style-type: none;
}

ol, .v-application ul {
  padding-left: 0px;
}

.blm-footer-year {
  margin-bottom: 0px;
}
.blm-footer-year-col {
  padding-bottom: 0px;
}
</style>
