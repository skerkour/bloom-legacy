<template>
  <v-data-table
    :headers="headers"
    :items="contributors"
    :loading="is_loading"
    hide-actions
  >
    <template slot="items" slot-scope="props">
      <td class="text-xs-left">
        <a :href="`https://github.com/${props.item.github_login}`" target="_blank" rel="noopener">
          {{ props.item.github_login }}
        </a>
      </td>
      <td class="text-xs-left">{{ props.item.commits }}</td>
    </template>
  </v-data-table>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';
const { log } = require('@bloom42/astro');

@Component
export default class Footer extends Vue {
  // props
  // data
  contributors: any[]  = [];
  is_loading = false;
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Name',
      value: 'github_login',
    },
    {
      align: 'left',
      sortable: false,
      text: 'Commits',
      value: 'commits',
    },
  ];


  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.is_loading = true;
    const client = api.client();
    try {
      const res = await client.get('/bloom/contributors');
      this.contributors = res.data.data;
      log.with({ contributors: this.contributors }).info('contributors successfully fetched');
    } catch (err) {
      log.with({ err }).error('loading contributors data');
    } finally {
      this.is_loading = false;
    }
  }
}
</script>


<style scoped lang="scss">
a {
  text-decoration: none;
}
</style>
