<template>
  <v-container fluid class="text-left">

    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row v-if="initialLoading" class="justify-center text-center">
      <v-col cols="12">
        <v-progress-circular
          indeterminate
          color="primary"
          size="50"
        />
      </v-col>
    </v-row>

    <v-row v-if="me">
      <v-col xs="12" sm="10" md="8" xl="7">


        <v-row>
          <v-col cols="12">
            <div class="headline">
              Public Profile
            </div>
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12" sm="3">
            <v-subheader>Avatar</v-subheader>
          </v-col>
          <v-col cols="12" sm="9">
            <blm-myaccount-form-avatar :avatar="avatarUrl" />
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12" sm="3">
            <v-subheader>Username</v-subheader>
          </v-col>
          <v-col cols="12" sm="9">
            <v-text-field :value="$store.state.me.username" disabled label="Username" prefix="@" />
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12" sm="3">
            <v-subheader>Display name</v-subheader>
          </v-col>
          <v-col cols="12" sm="9">
            <blm-myaccount-form-displayname :display-name="me.displayName"
              @updated="displayNameUpdated"/>
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12" sm="3">
            <v-subheader>Bio</v-subheader>
          </v-col>
          <v-col cols="12" sm="9">
            <blm-myaccount-form-bio :bio="me.bio" @updated="bioUpdated" />
          </v-col>
        </v-row>


        <v-row>
          <v-col><v-divider/></v-col>
        </v-row>


        <v-row>
          <v-col cols="12">
            <div class="headline">
              Personal Information
            </div>
          </v-col>
        </v-row>


        <v-row>
          <v-col cols="12" sm="3">
            <v-subheader>Email</v-subheader>
          </v-col>
          <v-col cols="12" sm="9">
            <blm-myaccount-form-email :email="me.email" />
          </v-col>
        </v-row>


      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import EmailForm from '../components/email_form.vue';
import DisplayNameForm from '../components/display_name_form.vue';
import BioForm from '../components/bio_form.vue';
import AvatarForm from '../components/avatar_form.vue';
import * as models from '@/api/models';
import core from '@/core';
import { Method } from '@/core/users';
import { Mutations } from '@/store';


@Component({
  components: {
    'blm-myaccount-form-email': EmailForm,
    'blm-myaccount-form-displayname': DisplayNameForm,
    'blm-myaccount-form-bio': BioForm,
    'blm-myaccount-form-avatar': AvatarForm,
  },
})
export default class Profile extends Vue {
  // props
  // data
  initialLoading = false;
  error = '';
  me: models.User | null = null;

  // computed
  get avatarUrl() {
    return this.me?.avatarUrl ? this.me?.avatarUrl : '/assets/imgs/profile.jpg';
  }

  // lifecycle
  created() {
    this.fetchData();
  }

  // watch
  // methods
  async fetchData() {
    this.error = '';
    this.initialLoading = true;

    try {
      this.me = await core.call(Method.FetchMyProfile, core.Empty);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.initialLoading = false;
    }
  }

  displayNameUpdated(displayName: string) {
    this.me!.displayName = displayName;
    this.$store.commit(Mutations.UPDATE_DISPLAY_NAME.toString(), displayName);
  }

  bioUpdated(bio: string) {
    this.me!.bio = bio;
  }
}
</script>


<style lang="scss" scoped>
</style>
