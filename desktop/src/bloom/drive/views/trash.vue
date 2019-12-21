<template>
  <v-container fluid>
    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-toolbar flat dense>
      <v-spacer />

      <v-tooltip bottom>
        <template v-slot:activator="{ on: tooltip }">
          <v-btn icon v-on="tooltip">
            <v-icon>mdi-history</v-icon>
          </v-btn>
        </template>
        <span>Restore</span>
      </v-tooltip>

      <v-tooltip bottom>
        <template v-slot:activator="{ on: tooltip }">
          <v-btn icon v-on="tooltip">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </template>
        <span>Delete forever</span>
      </v-tooltip>

    </v-toolbar>

    <v-data-table
      :headers="headers"
      :items="trash"
      item-key="id"
      :loading="isLoading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          Trash is empty
        </p>
      </template>

      <template v-slot:item="{ item }" class="text-left">
        <td>
          <span>{{ item.name | truncate }}</span>
        </td>
        <td>
          <span>{{ item.updated_at | calendar }}</span>
        </td>
        <td>
          <span>{{ item.size | filesize }}</span>
        </td>
      </template>
    </v-data-table>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Trash extends Vue {
  // props
  // data
  error = '';
  isLoading = false;
  trash = [];
  headers = [
    {
      align: 'left',
      sortable: true,
      text: 'Name',
      value: 'name',
      width: '40%',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Last modified',
      value: 'updated_at',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Size',
      value: 'size',
    },
  ];

  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
