<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense id="blm-history-toolbar">
      <v-spacer />

      <v-tooltip bottom>
        <template v-slot:activator="{ on }">
          <v-btn icon @click="clearHistory" v-on="on">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </template>
        <span>Clear history</span>
      </v-tooltip>
    </v-toolbar>


    <v-data-table
      :headers="headers"
      :items="history"
      item-key="id"
      :loading="isLoading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No downloads in history.
        </p>
      </template>

      <template v-slot:item="{ item }">
        <td class="text-left">
          <span>{{ item.name | truncate }}</span>
        </td>
        <td class="text-left">
          <span>{{ item.created_at | calendar }}</span>
        </td>
      </template>
    </v-data-table>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class History extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
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
  // watch
  // methods
  clearHistory() {
  }
}
</script>


<style lang="scss" scoped>
#blm-history-toolbar {
  border-bottom: 1px solid rgba($color: #000000, $alpha: 0.1);
}
</style>
