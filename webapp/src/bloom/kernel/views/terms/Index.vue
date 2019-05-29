<template>
  <div>
  <v-container class="mt-4">
    <v-layout row wrap justify-center>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-4">
        <h1 class="display-1 font-weight-medium mb-3">
          Terms of Service
        </h1>
        <h4 class="text--darken-1 mb-3">
              Last updated: May 29, 2019
        </h4>

        <p class="font-18 font-weight-regular mt-3">
          READ THESE TERMS CAREFULLY BEFORE BROWSING THIS WEBSITE.
          USING THIS WEBSITE INDICATES THAT YOU HAVE BOTH READ AND ACCEPT THESE TERMS.
          YOU CANNOT USE THIS WEBSITE IF YOU DO NOT ACCEPT THESE TERMS.
        </p>

      </v-flex>


      <v-flex xs12 sm10 md8 class="text-xs-left mb-4">
        <v-card class="elevation-0">
          <v-card-text class="font-18 font-weight-regular">
            <p>
              The following Terms of Services ("ToS") govern your relationship ("Customer" or "You")
              with <i>{{ COMPANY }}</i> ("Phaser", "Bloom", "we", or "us"), a Société par Actions Simplifiée registered in
              Grenoble, France, under number {{ SIRET }} having its registered office at
              107 Chemin des vergers, 38190 Bernin, France.
            </p>

            <h4 class="text-xs-left text--darken-1 mt-3 mb-3">
              Revisions and Errata
            </h4>
            <p>
              The materials appearing on Phaser's service could include technical or
              contents errors. Phaser does not warrant that any of the materials on
              its service are accurate, complete, or current. Phaser may make changes
              to the materials contained on its service at any time without notice.
            </p>

            <h4 class="text-xs-left text--darken-1 mt-3 mb-3">
              Scope
            </h4>
            <p>
                Phaser provides Services aimed at reinforcing the security of Customer’s Properties and protecting
                increasing customer's productivity.
                Phaser Services are subject to the terms and conditions of these ToS.
                You may access and use the Platform in accordance with these Terms. <br/><br/>

                By signing up with Phaser, you irrevocably agree to be bound by this ToS.
                You shall not use the Services if You do not accept the provisions of this ToS.<br/><br/>

                When using the features of the Platform you are subject to any agreements,
                policies or guidelines applicable to such features that may be posted from time to time.
                All such agreements, policies or guidelines, including the  <router-link to="/privacy">Privacy Policy</router-link>
                (as applicable) are hereby
                incorporated by reference into these Terms.
            </p>






          </v-card-text>
        </v-card>
      </v-flex>


    </v-layout>
  </v-container>
  <blm-footer />
</div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  COMPANY = 'Phaser Security SAS';
  SIRET = '839 477 668 00017';

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
