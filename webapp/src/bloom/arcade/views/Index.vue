<template>
<div v-if="$store.state.session">
  <v-container >
    <v-layout row>
      <v-flex xs12>
        <v-card class="elevation-0">
          <v-container
          fluid
          grid-list-md
          >
          <v-layout row wrap justify-center>
            <v-flex
            v-for="card in cards"
            xs4
            sm3
            lg2
            :key="card.title"
            >
            <v-tooltip bottom>
              <a :href="card.href" slot="activator">
                  <v-img
                  :src="card.logo"
                  height="150px"
                  contain
                  class="app-icon"
                  >
                </v-img>

                  <p class="headline text-xs-center app-name mt-3">{{ card.name }}</p>
            </a>
            <span>{{ card.tooltip }}</span>
          </v-tooltip>
        </v-flex>
      </v-layout>
    </v-container>
  </v-card>
</v-flex>
</v-layout>
</v-container>
</div>
  <blm-arcade-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';


@Component({
  components: {
    'blm-arcade-landing': Landing,
  },
})
export default class Index extends Vue {
  private cards = [
    {
      href: '/arcade/static/games/2048',
      logo: '/arcade/static/games/2048/logo.png',
      name: '2048',
      tooltip: 'A small clone of 1024',
    },
  ];
}
</script>


<style scoped lang="scss">
.v-card {
  border-radius: 8px;
}

.v-card__actions p {
  width: 100%;
}

a {
  text-decoration: none;
}

.app-icon {
  border-radius: 28px;
}

.app-name {
  color: #787878;
}
</style>
