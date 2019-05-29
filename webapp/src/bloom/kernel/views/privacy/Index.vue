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
              information. The following outlines our privacy policy.<br />
              <b>We do not, and never will share our user data for advertising or marketing traking reasons.
                Advertising is the disruption of aesthetics, the insults to your intelligence and the interruption of your train of thought.
              </b>
            </i></p>
            <p>
              All user data is safely stored in France, Europe on <a href="https://aws.amazon.com/" target="_blank" rel="noopener">AWS</a>. We do not transfer data outside
              of France.
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
              Respect of Privacy
            </h4>
            <p>
              Here's what we do to protect user privacy:
              <ul>
                <li>Before or at the time of collecting personal information, we will identify the purposes for which information is being collected.</li>
                <li>We will collect and use of personal information solely with the objective of fulfilling those purposes specified by us and for other compatible purposes, unless we obtain the consent of the individual concerned or as required by law.</li>
                <li>We will only retain personal information as long as necessary for the fulfillment of those purposes.</li>
                <li>We will collect personal information by lawful and fair means and, where appropriate, with the knowledge or consent of the individual concerned.</li>
                <li>Personal data should be relevant to the purposes for which it is to be used, and, to the extent necessary for those purposes, should be accurate, complete, and up-to-date.</li>
                <li>We will protect personal information by reasonable security safeguards against loss or theft, as well as unauthorized access, disclosure, copying, use or modification.</li>
                <li>We will make readily available to customers information about our policies and practices relating to the management of personal information.</li>
                <li>The user IP address is kept indefinitely in the event the user started a chat session with a website on Bloom. We are legally required by
                  the law of France to log those IPs in the event of a legal request (for a minimum duration of 1 year).
                  Though, we keep those IP address longer as we need to aggregate them to protect
                   our services against botnets and spam attacks, which occur frequently.
                   The Bloom service could not function at the level our customers expect from us without statistics on those collected IP.</li>
              </ul>

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
