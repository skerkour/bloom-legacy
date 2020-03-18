<template>
  <div>
    <v-data-table
      :headers="headers"
      :items="groups"
      item-key="id"
      :loading="loading"
      loading-text="Loading... Please wait"
      hide-default-footer>
      <template v-slot:no-data>
        <p class="text-center">
          No group
        </p>
      </template>

      <template v-slot:item="{ item }">
        <tr>
          <td>
            <span>{{ item.name }}</span>
          </td>
          <td>
            <v-tooltip bottom>
              <template v-slot:activator="{ on }">
                <v-btn icon v-on="on" :to="`${inspectUrl}/${item.id}`">
                  <v-icon>mdi-magnify</v-icon>
                </v-btn>
              </template>
              <span>Inspect group</span>
            </v-tooltip>
          </td>
        </tr>
      </template>
    </v-data-table>
  </div>
</template>


<script lang="ts">
import { Component, Vue, Prop } from 'vue-property-decorator';
import { Group } from '@/api/models';

@Component
export default class GroupsSimpleTable extends Vue {
  // props
  @Prop({ type: String, default: '' }) inspectUrl!: string;
  @Prop({ type: Array, default: [] }) groups!: Group[];
  @Prop({ type: Boolean, default: false }) loading!: boolean;

  // data
  headers = [
    {
      align: 'left',
      sortable: false,
      text: 'Name',
      value: 'name',
    },
    { text: 'Actions', sortable: false },
  ];
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
