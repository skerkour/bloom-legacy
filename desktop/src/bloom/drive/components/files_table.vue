<template>
  <v-container fluid>
    <v-data-table
      :headers="headers"
      :items="files"
      item-key="id"
      :loading="loading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No file
        </p>
      </template>

      <template v-slot:item="{ item }" class="text-left">
        <td>
          <span>{{ item.name }}</span>
        </td>
        <td>
          <span>{{ item.updated_at | date }}</span>
        </td>
        <td>
          <span>{{ item.size | filesize }}</span>
        </td>
      </template>
    </v-data-table>
  </v-container>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class FilesTable extends Vue {
  // props
  @Prop({ type: Boolean, default: false }) loading!: boolean;
  @Prop({ type: Array }) files!: any[];

  // data
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
  // files
}
</script>


<style lang="scss" scoped>
</style>
