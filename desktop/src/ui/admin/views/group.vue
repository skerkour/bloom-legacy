<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row class="text-left" v-if="group">
      <v-col cols="12">
        <h1>{{ group.name }}</h1>
        <v-subheader>{{ group.id }}</v-subheader>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Group } from '@/api/models';
import { FetchGroupDetailsParams, Method } from '@/core/groups';
import core from '@/core';

@Component
export default class AdminGroupView extends Vue {
  // props
  // data
  error = '';
  group: Group | null = null;
  loading = false;
  groupId = '';

  // computed
  // lifecycle
  created() {
    this.groupId = this.$route.params.groupId;
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;
    const params: FetchGroupDetailsParams = {
      id: this.groupId,
    };

    try {
      this.group = await core.call(Method.FetchGroupDetails, params);
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
