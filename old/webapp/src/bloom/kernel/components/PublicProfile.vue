<template>
  <div>
  <v-container class="text-xs-center">
    <v-layout row wrap justify-center>

      <v-flex xs12 v-if="is_loading">
        <div>
          <v-progress-circular
            :size="50"
            color="primary"
            indeterminate
          ></v-progress-circular>
        </div>
      </v-flex>

      <v-flex xs12 text-xs-center v-if="error">
        <v-alert icon="mdi-alert-circle" :value="error" type="error">
          {{ error }}
        </v-alert>
      </v-flex>

      <v-flex xs12 v-if="profile">
        <v-avatar size="84" color="grey lighten-4">
         <img :src="profile.avatar_url" alt="avatar">
        </v-avatar>
        <h1 class="mt-2 headline">{{ profile.display_name }}</h1>
        <h5 class="mt-2 title">@{{ profile.username }}</h5>
        <p class="mt-2 subheading blm-bio">{{ profile.bio }}</p>
      </v-flex>

    </v-layout>
  </v-container>
</div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';

@Component({})
export default class PublicProfile extends Vue {
  // props
  // data
  profile: any = null;
  is_loading = false;
  error = '';


  // computed
  // lifecycle
  created() {
    this.fetch_data();
  }


  // watch
  // methods
  async fetch_data() {
    this.error = '';
    try {
      this.is_loading = true;
      const res = await api.get(`${api.MYACCOUNT}/v1/users/${this.$route.params.username}`);
      this.profile = res;
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

}
</script>

<style scoped lang="scss">
.blm-bio {
  max-width: 600px;
  margin: auto;
  word-break: break-all;
}
</style>
