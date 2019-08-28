<template>
  <v-footer height="auto" class="text-xs-left">
    <v-container>
      <v-layout row wrap class="text-xs-center text-sm-left">
        <v-flex xs12 sm4 class="pa-4">
          <h5 class="title pb-3">Organization</h5>
          <ul>
            <li><router-link class="nav-link" to="/about">About</router-link></li>
            <li><router-link class="nav-link" to="/contact">Contact</router-link></li>
            <li><router-link class="nav-link" to="/security">Security</router-link></li>
            <li><router-link class="nav-link" to="/terms">Terms</router-link></li>
            <li><router-link class="nav-link" to="/privacy">Privacy</router-link></li>
            <li><router-link class="nav-link" to="/become-a-sponsor">Sponsoring</router-link></li>
          </ul>
        </v-flex>
        <v-flex xs12 sm4 class="pa-4">
          <h5 class="title pb-3">Product</h5>
          <ul>
            <li><router-link to="/help">Help & Support</router-link></li>
            <li><router-link class="nav-link" to="/download">Download Apps</router-link></li>
            <li><a class="nav-link" href="https://gitlab.com/bloom42/docs/blob/master/contributing.md" target="_blank" rel="noopener">Contribute</a></li>
            <li><router-link to="/help">Documentation</router-link></li>
          </ul>
        </v-flex>
        <v-flex xs12 sm4 class="pa-4">
          <h5 class="title pb-3">Community</h5>
          <ul>
            <li><a class="nav-link" href="https://theguide.bloom.sh" target="_blank" rel="noopener">The Guide</a></li>
            <li><router-link class="nav-link" to="/open-source">Open Source</router-link></li>
            <li><a class="nav-link" href="https://twitter.com/42bloom" target="_blank" rel="noopener">Twitter</a></li>
            <li><a class="nav-link" href="https://discord.gg/HmDQDGv" target="_blank" rel="noopener">Discord Chat</a></li>
            <li><a class="nav-link" href="https://gitlab.com/bloom42" target="_blank" rel="noopener">GitLab</a></li>
          </ul>
        </v-flex>

        <v-flex xs12>
          <p class="text-xs-center">
            <span><a href="mailto:hello@bloom.sh">hello@bloom.sh</a></span><br/><br/>
            <span>&copy; 2018 - {{ new Date().getFullYear() }} Bloom</span>
          </p>
        </v-flex>

      </v-layout>
    </v-container>

    </v-footer>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


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

ol, ul {
  padding-left: 0px;
}
</style>
