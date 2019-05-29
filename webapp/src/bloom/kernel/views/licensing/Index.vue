<template>
  <v-container>
    <v-layout row>
      <v-flex xs12>
          <v-container fluid grid-list-md>
            <v-layout row wrap>

              <v-flex xs12 class="mb-5 title blue-grey--text text--darken-1">
                <h1>
                  Licensing
                </h1>
              </v-flex>

               <v-flex xs12 class="mb-3 text-xs-left">
                <p>
                  You can find the license for each project in the <code>LICENSE.txt</code> file
                  at the root of each project's repository.
                </p>
                <p>
                  Our software is released under the following licenses:
                  <ul>
                    <li>Apache v2.0 (Apache-2.0)</li>
                    <li>GNU Affero General Public License v3.0 (AGPL-3.0)</li>
                  </ul>
                </p>

                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Apache v2.0
                </h2>
                <p>
                  Used for libraries, drivers etc... you can use it freely according to the license.
                </p>


                <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  GNU Affero General Public License v3.0 (AGPL-3.0)
                </h2>
                <p>
                  We use the AGPL v3.0 license for larger software to
                  avoid behaviors which will damage our vision of Open Source/free software and ensure
                  that thos who benefits from our software contribute back to the community
                  by releasing the modified
                  version and the tooling arround, or by funding the project.
                  <br/><br/>
                  <!-- To be clear, creating a SaaS or IaaS or PaaS or hosting for fees or whatever
                  arround our software triggers the
                  AGPL v3.0 or SSPLv1.0 licenses while selling consulting / support services related
                  to the Software or self hosting does not.
                  <br/>
                  You can read more
                  <a class="nav-link" href="https://www.mongodb.com/community/licensing"
                    target="_blank" rel="noopener">Here</a>
                  <br/>
                  and <a class="nav-link" href="https://redislabs.com/community/licenses/"
                    target="_blank" rel="noopener">Here</a> -->
                </p>


                <!-- <h2 class="text-xs-center blue-grey--text text--darken-1 mt-5 mb-3">
                  Commercial licenses
                </h2>
                <p>
                  If use of our software does not satisfy your organization’s legal department,
                  commercial licenses are available with.
                  Feel free to
                  <a class="nav-link" href="https://bloom.sh/contact" target="_blank"
                  rel="noopener">contact us</a>
                  for more details.
                </p> -->
              </v-flex>


            </v-layout>
          </v-container>
      </v-flex>
    </v-layout>
  </v-container>
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
p {
  font-size: 16px;
}
</style>
