<template>
  <v-container fluid class="text-left">
    <v-row>
      <v-alert type="error" :value="error !== ''">
        {{ error }}
      </v-alert>
    </v-row>

    <v-row>
      <v-toolbar flat dense>
        <v-btn color="primary">
          <v-icon left>mdi-plus</v-icon>New
        </v-btn>
      </v-toolbar>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Group } from '@/api/models';
import core from '@/core';
import { Method, Groups } from '@/core/groups';

@Component
export default class GroupsView extends Vue {
  // props
  // data
  error = '';
  groups: Group[] = [];
  loading = false;

  // computed
  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;

    try {
      const res: Groups = await core.call(Method.FindGroups, core.Empty);
      this.groups = res.groups;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.loading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
