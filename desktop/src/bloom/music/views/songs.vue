<template>
  <v-container fluid>

    <v-data-table
      :headers="headers"
      :items="songs"
      item-key="id"
      :loading="isLoading"
      loading-text="Loading... Please wait"
      hide-default-footer
      class="text-left">
      <template v-slot:no-data>
        <p class="text-center">
          No song.
        </p>
      </template>

      <template v-slot:item="{ item }">
        <td>
          <span>{{ item.name | truncate }}</span>
        </td>
        <td>
          <span>{{ item.artist | truncate }}</span>
        </td>
        <td>
          <span>{{ item.album | truncate }}</span>
        </td>
        <td>
          <span>{{ item.duration }}</span>
        </td>
      </template>
    </v-data-table>

  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Songs extends Vue {
  // props
  // data
  isLoading = false;
  songs = [];
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
      text: 'Artist',
      value: 'artist',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Album',
      value: 'album',
    },
    {
      align: 'left',
      sortable: true,
      text: 'Time',
      value: 'duration',
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
