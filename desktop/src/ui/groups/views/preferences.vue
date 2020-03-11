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
        <v-btn color="error" @click="openDeleteGroupDialog" :loading="loading">
          <v-icon left>mdi-delete</v-icon>
          Delete Group
        </v-btn>
      </v-col>
    </v-row>

    <v-dialog v-model="deleteGroupDialog" max-width="300">
      <v-card class="text-left">
        <v-card-title class="headline">Delete Group?</v-card-title>

        <v-card-text>
          Are you sure you want to delete group?
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>

          <v-btn text @click="closeDeleteGroupDialog">
            Cancel
          </v-btn>

          <v-btn color="error" @click="deleteGroup">
            <v-icon left>mdi-delete</v-icon> Delete
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
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
  deleteGroupDialog = false;

  // computed
  created() {
    this.groupId = this.$route.params.group_id;
  }

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

  openDeleteGroupDialog() {
    this.deleteGroupDialog = true;
  }

  closeDeleteGroupDialog() {
    this.deleteGroupDialog = false;
  }
}
</script>


<style lang="scss" scoped>
</style>
