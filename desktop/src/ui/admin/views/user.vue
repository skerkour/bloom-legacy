<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>


    <v-row class="text-left" v-if="user">
      <v-col cols="12">
        <h1>@{{ user.username }}</h1>
      </v-col>

      <v-col cols="12">
        <h1>Info</h1>
        <!-- TODO -->
      </v-col>

      <v-col cols="12">
        <h1>Billing</h1>
        <!-- TODO -->
      </v-col>

      <v-col cols="12">
        <h1>Groups</h1>
        <!-- TODO: table -->
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { User } from '../../../api/models';
import { FetchUserParams, Method } from '@/core/users';
import core from '@/core';

@Component
export default class AdminUserView extends Vue {
  // props
  // data
  error = '';
  user: User | null = null;
  loading = false;
  username = '';

  // computed
  // lifecycle
  created() {
    this.username = this.$route.params.username;
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.loading = true;
    const params: FetchUserParams = {
      username: this.username,
    };

    try {
      this.user = await core.call(Method.FetchUserDetails, params);
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
