<template>
  <v-container fluid class="text-left">
    <v-row>
      <h1>Preferences</h1>
    </v-row>

    <v-row>
      <v-col cols="12" v-if="error">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
      <v-col cols="12" v-if="loading">
        <v-progress-circular
          indeterminate
          color="primary"
          size="50"
        />
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <v-btn color="error" @click="deleteGroup" :loading="loading">
          <v-icon left>mdi-delete</v-icon>
          Delete Group
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { DeleteGroupInput } from '../../../api/models';
import core from '@/core';
import { Method } from '@/core/groups';

@Component
export default class Groups extends Vue {
  // props
  // data
  loading = false;
  error = '';
  groupId = '';

  // computed
  created() {
    this.groupId = this.$route.params.group_id;
  }

  // TODO: DELETE GROUP ALERT

  // lifecycle
  // watch
  // methods
  async deleteGroup() {
    this.loading = true;
    this.error = '';
    const params: DeleteGroupInput = {
      id: this.groupId,
    };

    try {
      await core.call(Method.DeleteGroup, params);
      this.$router.push({ path: '/groups' });
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
