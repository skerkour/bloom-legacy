<template>
  <div>
    <v-alert icon="mdi-alert-circle" :value="error" type="error" dismissible>
      {{ error }}
    </v-alert>

    <v-toolbar flat dense class=" hidden-sm-and-down">
      <v-spacer></v-spacer>
      <v-toolbar-items>
        <v-tooltip bottom>
          <v-btn slot="activator" icon :disabled="history.length === 0" @click="clear_history">
            <v-icon color="grey darken-1">mdi-delete</v-icon>
          </v-btn>
          <span>Clear history</span>
        </v-tooltip>
      </v-toolbar-items>
    </v-toolbar>

    <v-data-table
    :headers="headers"
    :items="history"
    item-key="id"
    hide-actions
    :loading="is_loading">
    <template slot="no-data">
      <p class="text-xs-center">
        No history.
      </p>
    </template>
    <template slot="items" slot-scope="props">
      <td class="text-xs-left">
        <span>{{ props.item.name | truncate }}</span>
      </td>
      <td class="text-xs-left">
        <span>{{ props.item.created_at | calendar }}</span>
      </td>
    </template>
</v-data-table>

</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class History extends Vue {
  // props
  // data
  error = '';
  is_loading = false;
  history = [];
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
      width: '50%',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Date',
      value: 'created_at',
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
    this.error = '';
    this.is_loading = true;
    try {
      this.history = await api.get(`${api.BITFLOW}/v1/history`);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  async clear_history() {
    this.error = '';
    this.is_loading = true;
    try {
      await api.delete(`${api.BITFLOW}/v1/history`);
      this.history = [];
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }
}
</script>
