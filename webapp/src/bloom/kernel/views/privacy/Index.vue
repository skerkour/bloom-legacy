<template>
  <div>
  <v-container class="mt-4 text-xs-center">
    <v-layout row wrap justify-center>

      <v-flex xs12 sm10 md8 class="text-xs-center mb-4">
        <h1 class="display-1 font-weight-medium mb-3">
          Privacy Policy
        </h1>
        <h4 class="text-xs-center text--darken-1 mb-3">
              Last updated: May 29, 2019
        </h4>

      </v-flex>


      <v-flex xs12 sm10 md8 class="text-xs-left mb-4">
        <v-card class="elevation-0">
          <v-card-text class="font-18 font-weight-regular">
            <p class="mb-3"><i>
              We aim for transparency about how we collect data. Accordingly, we have developed this Policy in order
              for you to understand how we collect and make use of personal
              information. The following outlines our privacy policy.
            </i></p>
            <p>
              All user data is safely stored in France, Europe. We do not transfer data outside
              of the France.
            </p>
            <h4 class="text-xs-left text--darken-1 mt-3 mb-3">
              Owner and Data Controller
            </h4>
            <p>
              PHASER SECURITY SAS <br/>
              107 chemin des vergers, 38190 Bernin, France <br/> <br/>
              Owner contact email: <a href="mailto:hello@bloom.sh">hello@bloom.sh</a>
            </p>



            <h4 class="text-xs-left text--darken-1 mt-3 mb-3">
              Handling payments
            </h4>
            <p>
              We do not process financial data ourself and rely on an external payment provider instead.<br/>

              Payment processing services enable Bloom to process payments by credit card. To ensure
              greater security, Bloom shares only the information necessary to execute the transaction
              with the financial intermediaries handling the transaction.
            </p>
            <h5 class="text-xs-left text--darken-1 mt-3 mb-3">
              Stripe (Stripe Inc)
            </h5>
            <p>
              Stripe is a payment service provided by Stripe Inc.<br/>
            Personal Data collected: various types of Data as specified in the privacy policy of the service.<br/>
            Place of processing: United States – <a href="https://stripe.com/terms/US" target="_blank" rel="noopener">Privacy Policy</a>.
            Privacy Shield participant. <br/>
            </p>




            <p class="mt-5 mb-3"><i>
              We are committed to provide our services in accordance with these principles in
              order to ensure that the confidentiality of personal information is protected and maintained.
            </i></p>

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
